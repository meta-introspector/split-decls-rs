// Generated macro for impl_49 (impl)
macro_rules! Depcrate_testimpl_49 {
() => {
// Module: crate::test
// Provides: {"impl_49"}
// Dependencies: {}
impl < A : AsRef < str > > FromIterator < A > for TestCase { fn from_iter < T : IntoIterator < Item = A > > (iter : T) -> Self { TestCase { args : iter . into_iter () . map (expr) . collect () , attrs : Default :: default () , description : None , } } }
};
}
