// Generated macro for impl_135 (impl)
macro_rules! Depcrate_filesimpl_135 {
() => {
// Module: crate::files
// Provides: {"impl_135"}
// Dependencies: {}
impl < T > From < InRealFile < T > > for InFile < T > { fn from (InRealFile { file_id , value } : InRealFile < T >) -> Self { InFile { file_id : file_id . into () , value } } }
};
}
