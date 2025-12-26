use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn yokeable_derive_impl(input: &DeriveInput) -> TokenStream2 {
    let tybounds = input
        .generics
        .type_params()
        .map(|ty| {
            let mut ty = ty.clone();
            ty.eq_token = None;
            ty.default = None;
            ty
        })
        .collect::<Vec<_>>();
    let typarams = tybounds
        .iter()
        .map(|ty| ty.ident.clone())
        .collect::<Vec<_>>();
    let wherebounds = input
        .generics
        .where_clause
        .iter()
        .flat_map(|wc| wc.predicates.iter())
        .filter(|p| matches!(p, WherePredicate::Type(_)))
        .collect::<Vec<_>>();
    let static_bounds: Vec<WherePredicate> = typarams
        .iter()
        .map(|ty| parse_quote!(# ty : 'static))
        .collect();
    let lts = input.generics.lifetimes().count();
    if lts == 0 {
        let name = &input.ident;
        quote! {
            unsafe impl <'a, # (# tybounds),*> yoke::Yokeable <'a > for # name <# (#
            typarams),*> where # (# static_bounds,) * # (# wherebounds,) * Self : Sized {
            type Output = Self; #[inline] fn transform(& self) -> & Self::Output { self }
            #[inline] fn transform_owned(self) -> Self::Output { self } #[inline] unsafe
            fn make(this : Self::Output) -> Self { this } #[inline] fn transform_mut < F
            > (&'a mut self, f : F) where F : 'static + for <'b > FnOnce(&'b mut
            Self::Output) { f(self) } }
        }
    } else {
        if lts != 1 {
            return syn::Error::new(
                input.generics.span(),
                "derive(Yokeable) cannot have multiple lifetime parameters",
            )
            .to_compile_error();
        }
        let name = &input.ident;
        let manual_covariance = input.attrs.iter().any(|a| {
            if let Ok(i) = a.parse_args::<Ident>() {
                if i == "prove_covariance_manually" {
                    return true;
                }
            }
            false
        });
        if manual_covariance {
            let mut structure = Structure::new(input);
            let generics_env = typarams.iter().cloned().collect();
            let static_bounds: Vec<WherePredicate> = typarams
                .iter()
                .map(|ty| parse_quote!(# ty : 'static))
                .collect();
            let mut yoke_bounds: Vec<WherePredicate> = vec![];
            structure.bind_with(|_| synstructure::BindStyle::Move);
            let owned_body = structure.each_variant(|vi| {
                vi.construct(|f, i| {
                    let binding = format!("__binding_{i}");
                    let field = Ident::new(&binding, Span::call_site());
                    let fty_static = replace_lifetime(&f.ty, static_lt());
                    let (has_ty, has_lt) = visitor::check_type_for_parameters(&f.ty, &generics_env);
                    if has_ty {
                        if has_lt {
                            let fty_a = replace_lifetime(&f.ty, custom_lt("'a"));
                            yoke_bounds.push(parse_quote!(
                                # fty_static : yoke::Yokeable <'a, Output = # fty_a >
                            ));
                        } else {
                            yoke_bounds.push(parse_quote!(
                                # fty_static : yoke::Yokeable <'a, Output = # fty_static >
                            ));
                        }
                    }
                    if has_ty || has_lt {
                        quote! {
                            <# fty_static as yoke::Yokeable <'a >>::transform_owned(#
                            field)
                        }
                    } else {
                        quote! {
                            # field
                        }
                    }
                })
            });
            let borrowed_body = structure.each(|binding| {
                let f = binding.ast();
                let field = &binding.binding;
                let (has_ty, has_lt) = visitor::check_type_for_parameters(&f.ty, &generics_env);
                if has_ty || has_lt {
                    let fty_static = replace_lifetime(&f.ty, static_lt());
                    let fty_a = replace_lifetime(&f.ty, custom_lt("'a"));
                    quote! {
                        let _ : &# fty_a = &<# fty_static as yoke::Yokeable <'a
                        >>::transform(# field);
                    }
                } else {
                    quote! {}
                }
            });
            return quote! {
                unsafe impl <'a, # (# tybounds),*> yoke::Yokeable <'a > for # name
                <'static, # (# typarams),*> where # (# static_bounds,) * # (#
                wherebounds,) * # (# yoke_bounds,) * { type Output = # name <'a, # (#
                typarams),*>; #[inline] fn transform(&'a self) -> &'a Self::Output { if
                false { match self { # borrowed_body } } unsafe {
                ::core::mem::transmute(self) } } #[inline] fn transform_owned(self) ->
                Self::Output { match self { # owned_body } } #[inline] unsafe fn
                make(this : Self::Output) -> Self { use core:: { mem, ptr };
                debug_assert!(mem::size_of::< Self::Output > () == mem::size_of::< Self >
                ()); let ptr : * const Self = (& this as * const Self::Output).cast();
                #[allow(forgetting_copy_types, clippy::forget_copy,
                clippy::forget_non_drop, clippy::mem_forget)] mem::forget(this);
                ptr::read(ptr) } #[inline] fn transform_mut < F > (&'a mut self, f : F)
                where F : 'static + for <'b > FnOnce(&'b mut Self::Output) { unsafe {
                f(core::mem::transmute::<&'a mut Self, &'a mut Self::Output > (self)) } }
                }
            };
        }
        quote! {
            unsafe impl <'a, # (# tybounds),*> yoke::Yokeable <'a > for # name <'static,
            # (# typarams),*> where # (# static_bounds,) * { type Output = # name <'a, #
            (# typarams),*>; #[inline] fn transform(&'a self) -> &'a Self::Output { self
            } #[inline] fn transform_owned(self) -> Self::Output { self } #[inline]
            unsafe fn make(this : Self::Output) -> Self { use core:: { mem, ptr };
            debug_assert!(mem::size_of::< Self::Output > () == mem::size_of::< Self >
            ()); let ptr : * const Self = (& this as * const Self::Output).cast();
            #[allow(forgetting_copy_types, clippy::forget_copy, clippy::forget_non_drop,
            clippy::mem_forget)] mem::forget(this); ptr::read(ptr) } #[inline] fn
            transform_mut < F > (&'a mut self, f : F) where F : 'static + for <'b >
            FnOnce(&'b mut Self::Output) { unsafe { f(core::mem::transmute::<&'a mut
            Self, &'a mut Self::Output > (self)) } } }
        }
    }
}
