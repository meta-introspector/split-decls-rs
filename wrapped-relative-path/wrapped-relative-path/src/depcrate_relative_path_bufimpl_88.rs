// Generated macro for impl_88 (impl)
macro_rules! Depcrate_relative_path_bufimpl_88 {
() => {
// Module: crate::relative_path_buf
// Provides: {"impl_88"}
// Dependencies: {}
impl Hash for RelativePathBuf { # [inline] fn hash < H > (& self , h : & mut H) where H : Hasher , { self . as_relative_path () . hash (h) ; } }
};
}
