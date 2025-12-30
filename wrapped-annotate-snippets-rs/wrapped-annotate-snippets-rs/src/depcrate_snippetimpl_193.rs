// Generated macro for impl_193 (impl)
macro_rules! Depcrate_snippetimpl_193 {
() => {
// Module: crate::snippet
// Provides: {"impl_193"}
// Dependencies: {}
impl < 'a > From < & 'a String > for OptionCow < 'a > { fn from (value : & 'a String) -> Self { Self (Some (Cow :: Borrowed (value . as_str ()))) } }
};
}
