macro_rules! try_join_internal {
    () => {
        # [doc = " The `try_join!` macro."] # [proc_macro] pub fn try_join_internal (input : TokenStream) -> TokenStream { crate :: join :: try_join (input) }
    };
}

try_join_internal!()