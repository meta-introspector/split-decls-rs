// Generated macro for impl_7 (impl)
macro_rules! Depcrate_allocimpl_7 {
() => {
// Module: crate::alloc
// Provides: {"impl_7"}
// Dependencies: {}
impl < T > Bake for Vec < T > where T : Bake , { fn bake (& self , ctx : & CrateEnv) -> TokenStream { ctx . insert ("alloc") ; let data = self . iter () . map (| d | d . bake (ctx)) ; quote ! { alloc :: vec ! [# (# data ,) *] } } }
};
}
