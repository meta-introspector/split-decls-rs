// Generated macro for impl_166 (impl)
macro_rules! Depcrate_fs_filterimpl_166 {
() => {
// Module: crate::fs::filter
// Provides: {"impl_166"}
// Dependencies: {}
impl FromIterator < glob :: Pattern > for IgnorePatterns { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = glob :: Pattern > , { let patterns = iter . into_iter () . collect () ; Self { patterns } } }
};
}
