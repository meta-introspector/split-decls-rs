// Generated macro for impl_116 (impl)
macro_rules! Depcrateimpl_116 {
() => {
// Module: crate
// Provides: {"impl_116"}
// Dependencies: {}
# [doc = " [`AsRef<RelativePath>`] implementation for [`Component`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use relative_path::RelativePath;"] # [doc = ""] # [doc = " let mut it = RelativePath::new(\"../foo/bar\").components();"] # [doc = ""] # [doc = " let a = it.next().ok_or(\"a\")?;"] # [doc = " let b = it.next().ok_or(\"b\")?;"] # [doc = " let c = it.next().ok_or(\"c\")?;"] # [doc = ""] # [doc = " let a: &RelativePath = a.as_ref();"] # [doc = " let b: &RelativePath = b.as_ref();"] # [doc = " let c: &RelativePath = c.as_ref();"] # [doc = ""] # [doc = " assert_eq!(a, \"..\");"] # [doc = " assert_eq!(b, \"foo\");"] # [doc = " assert_eq!(c, \"bar\");"] # [doc = ""] # [doc = " # Ok::<_, Box<dyn std::error::Error>>(())"] # [doc = " ```"] impl AsRef < RelativePath > for Component < '_ > { # [inline] fn as_ref (& self) -> & RelativePath { self . as_str () . as_ref () } }
};
}
