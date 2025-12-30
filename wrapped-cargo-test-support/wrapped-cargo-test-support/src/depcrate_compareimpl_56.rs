// Generated macro for impl_56 (impl)
macro_rules! Depcrate_compareimpl_56 {
() => {
// Module: crate::compare
// Provides: {"impl_56"}
// Dependencies: {}
impl < const N : usize , P , D > From < [(P , D) ; N] > for InMemoryDir where P : Into < PathBuf > , D : IntoData , { fn from (files : [(P , D) ; N]) -> Self { let files = files . into_iter () . map (| (p , d) | (p . into () , d . into_data ())) . collect () ; Self { files } } }
};
}
