macro_rules! stream_select_internal {
    () => {
        # [doc = " The `stream_select!` macro."] # [proc_macro] pub fn stream_select_internal (input : TokenStream) -> TokenStream { crate :: stream_select :: stream_select (input . into ()) . unwrap_or_else (syn :: Error :: into_compile_error) . into () }
    };
}

stream_select_internal!()