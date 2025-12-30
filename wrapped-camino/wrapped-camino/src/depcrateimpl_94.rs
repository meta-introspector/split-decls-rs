// Generated macro for impl_94 (impl)
macro_rules! Depcrateimpl_94 {
() => {
// Module: crate
// Provides: {"impl_94"}
// Dependencies: {}
impl < 'a > From < & 'a Utf8Path > for Cow < 'a , Path > { fn from (path : & 'a Utf8Path) -> Cow < 'a , Path > { Cow :: Borrowed (path . as_ref ()) } }
};
}
