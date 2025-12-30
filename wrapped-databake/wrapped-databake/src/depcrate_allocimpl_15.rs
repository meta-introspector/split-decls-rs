// Generated macro for impl_15 (impl)
macro_rules! Depcrate_allocimpl_15 {
() => {
// Module: crate::alloc
// Provides: {"impl_15"}
// Dependencies: {}
impl < K , V > Bake for std :: collections :: HashMap < K , V > where K : Bake , V : Bake , { fn bake (& self , ctx : & CrateEnv) -> TokenStream { ctx . insert ("std") ; let mut data = self . iter () . map (| (k , v) | { let k = k . bake (ctx) ; let v = v . bake (ctx) ; quote ! ((# k , # v)) }) . collect :: < Vec < _ > > () ; data . sort_unstable_by_key (| data | data . to_string ()) ; quote ! { std :: collections :: HashMap :: from ([# (# data) ,*]) } } }
};
}
