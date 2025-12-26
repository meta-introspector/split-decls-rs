extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn quote_error(input: TokenStream) -> TokenStream {
    // For now, we just return the input tokens directly.
    // Later, this will parse the error session and generate structured data.
    input
}
