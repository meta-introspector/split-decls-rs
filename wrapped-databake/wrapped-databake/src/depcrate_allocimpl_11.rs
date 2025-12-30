// Generated macro for impl_11 (impl)
macro_rules! Depcrate_allocimpl_11 {
() => {
// Module: crate::alloc
// Provides: {"impl_11"}
// Dependencies: {}
impl < K , V > Bake for alloc :: collections :: BTreeMap < K , V > where K : Bake , V : Bake , { fn bake (& self , ctx : & CrateEnv) -> TokenStream { ctx . insert ("alloc") ; let data = self . iter () . map (| (k , v) | { let k = k . bake (ctx) ; let v = v . bake (ctx) ; quote ! ((# k , # v)) }) ; quote ! { alloc :: collections :: BTreeMap :: from ([# (# data) ,*]) } } }
};
}
