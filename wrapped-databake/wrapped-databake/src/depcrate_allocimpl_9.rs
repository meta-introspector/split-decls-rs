// Generated macro for impl_9 (impl)
macro_rules! Depcrate_allocimpl_9 {
() => {
// Module: crate::alloc
// Provides: {"impl_9"}
// Dependencies: {}
impl < T > Bake for alloc :: collections :: BTreeSet < T > where T : Bake , { fn bake (& self , ctx : & CrateEnv) -> TokenStream { ctx . insert ("alloc") ; let data = self . iter () . map (| d | d . bake (ctx)) ; quote ! { alloc :: collections :: BTreeSet :: from ([# (# data) ,*]) } } }
};
}
