// Generated macro for quote_index (function)
macro_rules! Depcrate_indexquote_index {
() => {
// Module: crate::index
// Provides: {"quote_index"}
// Dependencies: {}
pub (crate) fn quote_index (index : Index) -> proc_macro2 :: TokenStream { match index { Index :: Explicit (index) => quote ! (& sval :: Index :: from (# index)) , Index :: Implicit (index) => { quote ! (& sval :: Index :: from (# index) . with_tag (& sval :: tags :: VALUE_OFFSET)) } } }
};
}
