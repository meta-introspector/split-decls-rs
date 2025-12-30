// Generated macro for Localization (struct)
macro_rules! Depcrate_localizationLocalization {
() => {
// Module: crate::localization
// Provides: {"Localization"}
// Dependencies: {}
pub struct Localization < G , P > where G : BundleGenerator < LocalesIter = P :: Iter > , P : LocalesProvider , { bundles : OnceCell < Rc < Bundles < G > > > , generator : G , provider : P , sync : bool , res_ids : FxHashSet < ResourceId > , }
};
}
