// Generated macro for dbg_and_swallow (function)
macro_rules! Depcratedbg_and_swallow {
() => {
// Module: crate
// Provides: {"dbg_and_swallow"}
// Dependencies: {}
# [proc_macro] pub fn dbg_and_swallow (input : TokenStream) -> TokenStream { for token in input { println ! ("{} -> {:#?}" , token , Literal :: try_from (& token)) ; } TokenStream :: new () }
};
}
