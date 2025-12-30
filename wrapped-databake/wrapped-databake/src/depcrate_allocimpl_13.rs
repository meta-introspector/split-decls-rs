// Generated macro for impl_13 (impl)
macro_rules! Depcrate_allocimpl_13 {
() => {
// Module: crate::alloc
// Provides: {"impl_13"}
// Dependencies: {}
impl < T > Bake for std :: collections :: HashSet < T > where T : Bake , { fn bake (& self , ctx : & CrateEnv) -> TokenStream { ctx . insert ("std") ; let mut data = self . iter () . map (| d | d . bake (ctx)) . collect :: < Vec < _ > > () ; data . sort_unstable_by_key (| data | data . to_string ()) ; quote ! { std :: collections :: HashSet :: from ([# (# data) ,*]) } } }
};
}
