macro_rules! query_group {
    () => {
        # [proc_macro_attribute] pub fn query_group (args : TokenStream , input : TokenStream) -> TokenStream { match query_group_impl (args , input . clone ()) { Ok (tokens) => tokens , Err (e) => token_stream_with_error (input , e) , } }
    };
}

query_group!();