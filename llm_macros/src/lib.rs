use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn llm_error_message(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[proc_macro_attribute]
pub fn llm_context(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}
