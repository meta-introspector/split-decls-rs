// Generated macro for impl_228 (impl)
macro_rules! Depcrate_stringsimpl_228 {
() => {
// Module: crate::strings
// Provides: {"impl_228"}
// Dependencies: {}
impl < 'a > From < String > for CowStr < 'a > { fn from (s : String) -> Self { CowStr :: Boxed (s . into_boxed_str ()) } }
};
}
