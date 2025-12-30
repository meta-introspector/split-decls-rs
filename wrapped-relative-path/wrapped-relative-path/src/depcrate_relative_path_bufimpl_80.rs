// Generated macro for impl_80 (impl)
macro_rules! Depcrate_relative_path_bufimpl_80 {
() => {
// Module: crate::relative_path_buf
// Provides: {"impl_80"}
// Dependencies: {}
impl < 'a , T : ? Sized + AsRef < str > > From < & 'a T > for RelativePathBuf { # [inline] fn from (path : & 'a T) -> RelativePathBuf { RelativePathBuf { inner : path . as_ref () . to_owned () , } } }
};
}
