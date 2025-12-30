// Generated macro for impl_58 (impl)
macro_rules! Depcrate_compareimpl_58 {
() => {
// Module: crate::compare
// Provides: {"impl_58"}
// Dependencies: {}
impl < P , D > From < std :: collections :: BTreeMap < P , D > > for InMemoryDir where P : Into < PathBuf > , D : IntoData , { fn from (files : std :: collections :: BTreeMap < P , D >) -> Self { let files = files . into_iter () . map (| (p , d) | (p . into () , d . into_data ())) . collect () ; Self { files } } }
};
}
