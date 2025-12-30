// Generated macro for impl_63 (impl)
macro_rules! Depcrate_localizationimpl_63 {
() => {
// Module: crate::localization
// Provides: {"impl_63"}
// Dependencies: {}
impl < G , P > Localization < G , P > where G : BundleGenerator < LocalesIter = P :: Iter > + Default , P : LocalesProvider + Default , { pub fn new < I > (res_ids : I , sync : bool) -> Self where I : IntoIterator < Item = ResourceId > , { Self { bundles : OnceCell :: new () , generator : G :: default () , provider : P :: default () , sync , res_ids : FxHashSet :: from_iter (res_ids) , } } }
};
}
