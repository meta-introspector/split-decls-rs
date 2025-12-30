// Generated macro for impl_64 (impl)
macro_rules! Depcrate_localizationimpl_64 {
() => {
// Module: crate::localization
// Provides: {"impl_64"}
// Dependencies: {}
impl < G , P > Localization < G , P > where G : BundleGenerator < LocalesIter = P :: Iter > , P : LocalesProvider , { pub fn with_env < I > (res_ids : I , sync : bool , provider : P , generator : G) -> Self where I : IntoIterator < Item = ResourceId > , { Self { bundles : OnceCell :: new () , generator , provider , sync , res_ids : FxHashSet :: from_iter (res_ids) , } } pub fn is_sync (& self) -> bool { self . sync } pub fn add_resource_id < T : Into < ResourceId > > (& mut self , res_id : T) { self . res_ids . insert (res_id . into ()) ; self . on_change () ; } pub fn add_resource_ids (& mut self , res_ids : Vec < ResourceId >) { self . res_ids . extend (res_ids) ; self . on_change () ; } pub fn remove_resource_id < T : PartialEq < ResourceId > > (& mut self , res_id : T) -> usize { self . res_ids . retain (| x | ! res_id . eq (x)) ; self . on_change () ; self . res_ids . len () } pub fn remove_resource_ids (& mut self , res_ids : Vec < ResourceId >) -> usize { self . res_ids . retain (| x | ! res_ids . contains (x)) ; self . on_change () ; self . res_ids . len () } pub fn set_async (& mut self) { if self . sync { self . sync = false ; self . on_change () ; } } pub fn on_change (& mut self) { self . bundles . take () ; } }
};
}
