// Generated macro for BundleGenerator (trait)
macro_rules! Depcrate_generatorBundleGenerator {
() => {
// Module: crate::generator
// Provides: {"BundleGenerator"}
// Dependencies: {}
pub trait BundleGenerator { type Resource : Borrow < FluentResource > ; type LocalesIter : Iterator < Item = LanguageIdentifier > ; type Iter : Iterator < Item = FluentBundleResult < Self :: Resource > > ; type Stream : Stream < Item = FluentBundleResult < Self :: Resource > > ; fn bundles_iter (& self , _locales : Self :: LocalesIter , _res_ids : FxHashSet < ResourceId > ,) -> Self :: Iter { unimplemented ! () ; } fn bundles_stream (& self , _locales : Self :: LocalesIter , _res_ids : FxHashSet < ResourceId > ,) -> Self :: Stream { unimplemented ! () ; } }
};
}
