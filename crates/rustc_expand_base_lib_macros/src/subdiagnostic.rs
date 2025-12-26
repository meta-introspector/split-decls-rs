use proc_macro2::TokenStream;
use quote::quote;
use synstructure::Structure;

pub fn subdiagnostic_derive(s: Structure<'_>) -> TokenStream {
    let name = &s.ast().ident;

    // For now, a simple subdiagnostic with a message.
    let expanded = s.gen_impl(quote! {
        gen impl crate::Subdiagnostic for @Self {
            fn get_message(&self) -> String {
                format!("Subdiagnostic: {}", stringify!(#name))
            }
        }
    });

    expanded.into()
}
