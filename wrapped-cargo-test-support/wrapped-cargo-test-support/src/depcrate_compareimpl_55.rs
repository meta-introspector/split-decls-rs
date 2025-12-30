// Generated macro for impl_55 (impl)
macro_rules! Depcrate_compareimpl_55 {
() => {
// Module: crate::compare
// Provides: {"impl_55"}
// Dependencies: {}
impl < P , D > FromIterator < (P , D) > for InMemoryDir where P : Into < std :: path :: PathBuf > , D : IntoData , { fn from_iter < I : IntoIterator < Item = (P , D) > > (files : I) -> Self { let files = files . into_iter () . map (| (p , d) | (p . into () , d . into_data ())) . collect () ; Self { files } } }
};
}
