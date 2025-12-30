// Generated macro for impl_252 (impl)
macro_rules! Depcrate_hirimpl_252 {
() => {
// Module: crate::hir
// Provides: {"impl_252"}
// Dependencies: {}
impl AttrPath { pub fn from_ast (path : & ast :: Path) -> Self { AttrPath { segments : path . segments . iter () . map (| i | i . ident) . collect :: < Vec < _ > > () . into_boxed_slice () , span : path . span , } } }
};
}
