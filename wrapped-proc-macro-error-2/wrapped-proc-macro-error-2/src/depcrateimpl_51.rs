// Generated macro for impl_51 (impl)
macro_rules! Depcrateimpl_51 {
() => {
// Module: crate
// Provides: {"impl_51"}
// Dependencies: {}
impl < T > OptionExt for Option < T > { type Some = T ; fn expect_or_abort (self , message : & str) -> T { match self { Some (res) => res , None => abort_call_site ! (message) , } } }
};
}
