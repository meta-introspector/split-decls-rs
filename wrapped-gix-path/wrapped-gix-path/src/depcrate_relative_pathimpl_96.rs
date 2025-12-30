// Generated macro for impl_96 (impl)
macro_rules! Depcrate_relative_pathimpl_96 {
() => {
// Module: crate::relative_path
// Provides: {"impl_96"}
// Dependencies: {}
impl RelativePath { fn new_unchecked (value : & BStr) -> Result < & RelativePath , Error > { # [allow (unsafe_code)] unsafe { std :: mem :: transmute (value) } } }
};
}
