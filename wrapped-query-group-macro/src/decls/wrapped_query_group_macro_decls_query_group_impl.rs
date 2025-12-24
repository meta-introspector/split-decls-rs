use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub(crate) fn query_group_impl(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> Result<proc_macro::TokenStream, syn::Error> {
    let mut item_trait = syn::parse::<ItemTrait>(input)?;
    let supertraits = &item_trait.supertraits;
    let db_attr: Attribute = parse_quote! {
        #[salsa_macros::db]
    };
    item_trait.attrs.push(db_attr);
    let trait_name_ident = &item_trait.ident.clone();
    let input_struct_name = format_ident!("{}Data", trait_name_ident);
    let create_data_ident = format_ident!("create_data_{}", trait_name_ident);
    let mut input_struct_fields: Vec<InputStructField> = vec![];
    let mut trait_methods = vec![];
    let mut setter_trait_methods = vec![];
    let mut lookup_signatures = vec![];
    let mut lookup_methods = vec![];
    for item in &mut item_trait.items {
        if let syn::TraitItem::Fn(method) = item {
            let method_name = &method.sig.ident;
            let signature = &method.sig;
            let (_attrs, salsa_attrs) = filter_attrs(method.attrs.clone());
            let mut query_kind = QueryKind::TrackedWithSalsaStruct;
            let mut invoke = None;
            let mut cycle = None;
            let mut interned_struct_path = None;
            let mut lru = None;
            let params: Vec<FnArg> = signature.inputs.clone().into_iter().collect();
            let pat_and_tys = params
                .into_iter()
                .filter(|fn_arg| matches!(fn_arg, FnArg::Typed(_)))
                .map(|fn_arg| match fn_arg {
                    FnArg::Typed(pat_type) => pat_type,
                    FnArg::Receiver(_) => {
                        unreachable!("this should have been filtered out")
                    }
                })
                .collect::<Vec<syn::PatType>>();
            for SalsaAttr { name, tts, span } in salsa_attrs {
                match name.as_str() {
                    "cycle" => {
                        let c = syn::parse::<Parenthesized<Cycle>>(tts)?;
                        cycle = Some(c.0);
                    }
                    "input" => {
                        if !pat_and_tys.is_empty() {
                            return Err(
                                syn::Error::new(
                                    span,
                                    "input methods cannot have a parameter",
                                ),
                            );
                        }
                        query_kind = QueryKind::Input;
                    }
                    "interned" => {
                        let syn::ReturnType::Type(_, ty) = &signature.output else {
                            return Err(
                                syn::Error::new(
                                    span,
                                    "interned queries must have return type",
                                ),
                            );
                        };
                        let syn::Type::Path(path) = &**ty else {
                            return Err(
                                syn::Error::new(
                                    span,
                                    "interned queries must have return type",
                                ),
                            );
                        };
                        interned_struct_path = Some(path.path.clone());
                        query_kind = QueryKind::Interned;
                    }
                    "invoke_interned" => {
                        let path = syn::parse::<Parenthesized<Path>>(tts)?;
                        invoke = Some(path.0.clone());
                        query_kind = QueryKind::Tracked;
                    }
                    "invoke" => {
                        let path = syn::parse::<Parenthesized<Path>>(tts)?;
                        invoke = Some(path.0.clone());
                        if query_kind != QueryKind::Transparent {
                            query_kind = QueryKind::TrackedWithSalsaStruct;
                        }
                    }
                    "tracked" if method.default.is_some() => {
                        query_kind = QueryKind::TrackedWithSalsaStruct;
                    }
                    "lru" => {
                        let lru_count = syn::parse::<Parenthesized<syn::LitInt>>(tts)?;
                        let lru_count = lru_count.0.base10_parse::<u32>()?;
                        lru = Some(lru_count);
                    }
                    "transparent" => {
                        query_kind = QueryKind::Transparent;
                    }
                    _ => {
                        return Err(
                            syn::Error::new(span, format!("unknown attribute `{name}`")),
                        );
                    }
                }
            }
            let syn::ReturnType::Type(_, return_ty) = signature.output.clone() else {
                return Err(
                    syn::Error::new(signature.span(), "Queries must have a return type"),
                );
            };
            if let syn::Type::Path(ref ty_path) = *return_ty
                && matches!(query_kind, QueryKind::Input)
            {
                let field = InputStructField {
                    name: method_name.to_token_stream(),
                    ty: ty_path.path.to_token_stream(),
                };
                input_struct_fields.push(field);
            }
            if let Some(block) = &mut method.default {
                SelfToDbRewriter.visit_block_mut(block);
            }
            match (query_kind, invoke) {
                (QueryKind::Input, None) => {
                    let query = InputQuery {
                        signature: method.sig.clone(),
                        create_data_ident: create_data_ident.clone(),
                    };
                    let value = Queries::InputQuery(query);
                    trait_methods.push(value);
                    let setter = InputSetter {
                        signature: method.sig.clone(),
                        return_type: *return_ty.clone(),
                        create_data_ident: create_data_ident.clone(),
                    };
                    setter_trait_methods.push(SetterKind::Plain(setter));
                    let setter = InputSetterWithDurability {
                        signature: method.sig.clone(),
                        return_type: *return_ty.clone(),
                        create_data_ident: create_data_ident.clone(),
                    };
                    setter_trait_methods.push(SetterKind::WithDurability(setter));
                }
                (QueryKind::Interned, None) => {
                    let interned_struct_path = interned_struct_path.unwrap();
                    let method = Intern {
                        signature: signature.clone(),
                        pat_and_tys: pat_and_tys.clone(),
                        interned_struct_path: interned_struct_path.clone(),
                    };
                    trait_methods.push(Queries::Intern(method));
                    let mut method = Lookup {
                        signature: signature.clone(),
                        pat_and_tys: pat_and_tys.clone(),
                        return_ty: *return_ty,
                        interned_struct_path,
                    };
                    method.prepare_signature();
                    lookup_signatures
                        .push(
                            TraitItem::Fn(make_trait_method(method.signature.clone())),
                        );
                    lookup_methods.push(method);
                }
                (QueryKind::Tracked, invoke) => {
                    let method = TrackedQuery {
                        trait_name: trait_name_ident.clone(),
                        generated_struct: Some(GeneratedInputStruct {
                            input_struct_name: input_struct_name.clone(),
                            create_data_ident: create_data_ident.clone(),
                        }),
                        signature: signature.clone(),
                        pat_and_tys: pat_and_tys.clone(),
                        invoke,
                        cycle,
                        lru,
                        default: method.default.take(),
                    };
                    trait_methods.push(Queries::TrackedQuery(method));
                }
                (QueryKind::TrackedWithSalsaStruct, invoke) => {
                    let method = TrackedQuery {
                        trait_name: trait_name_ident.clone(),
                        generated_struct: None,
                        signature: signature.clone(),
                        pat_and_tys: pat_and_tys.clone(),
                        invoke,
                        cycle,
                        lru,
                        default: method.default.take(),
                    };
                    trait_methods.push(Queries::TrackedQuery(method))
                }
                (QueryKind::Transparent, invoke) => {
                    let method = Transparent {
                        signature: method.sig.clone(),
                        pat_and_tys: pat_and_tys.clone(),
                        invoke,
                        default: method.default.take(),
                    };
                    trait_methods.push(Queries::Transparent(method));
                }
                (QueryKind::Interned, Some(path)) => {
                    return Err(
                        syn::Error::new(
                            path.span(),
                            "Interned queries cannot be used with an `#[invoke]`"
                                .to_string(),
                        ),
                    );
                }
                (QueryKind::Input, Some(path)) => {
                    return Err(
                        syn::Error::new(
                            path.span(),
                            "Inputs cannot be used with an `#[invoke]`".to_string(),
                        ),
                    );
                }
            }
        }
    }
    let fields = input_struct_fields
        .into_iter()
        .map(|input| {
            let name = input.name;
            let ret = input.ty;
            quote! {
                # name : Option <# ret >
            }
        })
        .collect::<Vec<proc_macro2::TokenStream>>();
    let input_struct = quote! {
        #[salsa_macros::input] pub (crate) struct # input_struct_name { # (# fields),* }
    };
    let field_params = std::iter::repeat_n(
            quote! {
                None
            },
            fields.len(),
        )
        .collect::<Vec<proc_macro2::TokenStream>>();
    let create_data_method = quote! {
        #[allow(non_snake_case)] #[salsa_macros::tracked] fn # create_data_ident(db : &
        dyn # trait_name_ident) -> # input_struct_name { # input_struct_name::new(db, #
        (# field_params),*) }
    };
    let mut setter_signatures = vec![];
    let mut setter_methods = vec![];
    for trait_item in setter_trait_methods
        .iter()
        .map(|method| method.to_token_stream())
        .map(|tokens| syn::parse2::<syn::TraitItemFn>(tokens).unwrap())
    {
        let mut methods_sans_body = trait_item.clone();
        methods_sans_body.default = None;
        methods_sans_body.semi_token = Some(syn::Token![;](trait_item.span()));
        setter_signatures.push(TraitItem::Fn(methods_sans_body));
        setter_methods.push(TraitItem::Fn(trait_item));
    }
    item_trait.items.append(&mut setter_signatures);
    item_trait.items.append(&mut lookup_signatures);
    let trait_impl = quote! {
        #[salsa_macros::db] impl < DB > # trait_name_ident for DB where DB : #
        supertraits, { # (# trait_methods) * # (# setter_methods) * # (# lookup_methods)
        * }
    };
    RemoveAttrsFromTraitMethods.visit_item_trait_mut(&mut item_trait);
    let out = quote! {
        # item_trait # trait_impl # input_struct # create_data_method
    }
        .into();
    Ok(out)
}
