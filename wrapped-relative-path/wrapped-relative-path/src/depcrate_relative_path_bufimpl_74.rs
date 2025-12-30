// Generated macro for impl_74 (impl)
macro_rules! Depcrate_relative_path_bufimpl_74 {
() => {
// Module: crate::relative_path_buf
// Provides: {"impl_74"}
// Dependencies: {}
impl < 'a > From < & 'a RelativePath > for Cow < 'a , RelativePath > { # [inline] fn from (s : & 'a RelativePath) -> Cow < 'a , RelativePath > { Cow :: Borrowed (s) } }
};
}
