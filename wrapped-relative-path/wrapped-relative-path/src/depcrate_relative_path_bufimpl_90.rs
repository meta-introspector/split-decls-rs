// Generated macro for impl_90 (impl)
macro_rules! Depcrate_relative_path_bufimpl_90 {
() => {
// Module: crate::relative_path_buf
// Provides: {"impl_90"}
// Dependencies: {}
impl < P > FromIterator < P > for RelativePathBuf where P : AsRef < RelativePath > , { # [inline] fn from_iter < I > (iter : I) -> RelativePathBuf where I : IntoIterator < Item = P > , { let mut buf = RelativePathBuf :: new () ; buf . extend (iter) ; buf } }
};
}
