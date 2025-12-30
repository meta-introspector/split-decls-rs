// Generated macro for impl_92 (impl)
macro_rules! Depcrate_relative_path_bufimpl_92 {
() => {
// Module: crate::relative_path_buf
// Provides: {"impl_92"}
// Dependencies: {}
# [doc = " [`AsRef<str>`] implementation for [`RelativePathBuf`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use relative_path::RelativePathBuf;"] # [doc = ""] # [doc = " let path = RelativePathBuf::from(\"foo/bar\");"] # [doc = " let string: &str = path.as_ref();"] # [doc = " assert_eq!(string, \"foo/bar\");"] # [doc = " ```"] impl AsRef < str > for RelativePathBuf { # [inline] fn as_ref (& self) -> & str { & self . inner } }
};
}
