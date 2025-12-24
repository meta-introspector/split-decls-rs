use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn construct_take_rest(fields: &Fields) -> Result<TokenStream> {
    construct(
        fields,
        |idx, field| {
            determine_field_constructor(field)
                .map(|field_constructor| match field_constructor {
                    FieldConstructor::Default => {
                        quote!(::core::default::Default::default())
                    }
                    FieldConstructor::Arbitrary => {
                        if idx + 1 == fields.len() {
                            quote! {
                                arbitrary::Arbitrary::arbitrary_take_rest(u) ?
                            }
                        } else {
                            quote! {
                                arbitrary::Arbitrary::arbitrary(& mut u) ?
                            }
                        }
                    }
                    FieldConstructor::With(function_or_closure) => {
                        quote!((# function_or_closure) (& mut u) ?)
                    }
                    FieldConstructor::Value(value) => quote!(# value),
                })
        },
    )
}
