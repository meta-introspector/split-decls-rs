// Generated macro for impl_67 (impl)
macro_rules! Depcrate_localizationimpl_67 {
() => {
// Module: crate::localization
// Provides: {"impl_67"}
// Dependencies: {}
impl < G , P > Localization < G , P > where G : BundleGenerator < LocalesIter = P :: Iter > , P : LocalesProvider , { pub fn bundles (& self) -> & Rc < Bundles < G > > { self . bundles . get_or_init (| | { Rc :: new (Bundles :: new (self . sync , self . res_ids . clone () , & self . generator , & self . provider ,)) }) } }
};
}
