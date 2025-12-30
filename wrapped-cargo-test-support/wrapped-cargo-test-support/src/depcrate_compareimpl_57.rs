// Generated macro for impl_57 (impl)
macro_rules! Depcrate_compareimpl_57 {
() => {
// Module: crate::compare
// Provides: {"impl_57"}
// Dependencies: {}
impl < P , D > From < std :: collections :: HashMap < P , D > > for InMemoryDir where P : Into < PathBuf > , D : IntoData , { fn from (files : std :: collections :: HashMap < P , D >) -> Self { let files = files . into_iter () . map (| (p , d) | (p . into () , d . into_data ())) . collect () ; Self { files } } }
};
}
