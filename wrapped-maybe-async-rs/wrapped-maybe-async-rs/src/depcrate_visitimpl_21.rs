// Generated macro for impl_21 (impl)
macro_rules! Depcrate_visitimpl_21 {
() => {
// Module: crate::visit
// Provides: {"impl_21"}
// Dependencies: {}
impl AsyncAwaitRemoval { pub fn remove_async_await (& mut self , item : TokenStream) -> TokenStream { let mut syntax_tree : File = syn :: parse (item . into ()) . unwrap () ; self . visit_file_mut (& mut syntax_tree) ; quote ! (# syntax_tree) } }
};
}
