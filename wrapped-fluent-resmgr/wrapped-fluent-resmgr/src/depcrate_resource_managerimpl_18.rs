// Generated macro for impl_18 (impl)
macro_rules! Depcrate_resource_managerimpl_18 {
() => {
// Module: crate::resource_manager
// Provides: {"impl_18"}
// Dependencies: {}
impl BundleGenerator for ResourceManager { type Resource = FluentResource ; type LocalesIter = std :: vec :: IntoIter < LanguageIdentifier > ; type Iter = BundleIter ; type Stream = BundleIter ; fn bundles_iter (& self , locales : Self :: LocalesIter , res_ids : FxHashSet < ResourceId > ,) -> Self :: Iter { BundleIter { locales , res_ids } } fn bundles_stream (& self , _locales : Self :: LocalesIter , _res_ids : FxHashSet < ResourceId > ,) -> Self :: Stream { todo ! () } }
};
}
