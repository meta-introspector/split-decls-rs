// Generated macro for impl_558 (impl)
macro_rules! Depcrate_coverageinfo_ffiimpl_558 {
() => {
// Module: crate::coverageinfo::ffi
// Provides: {"impl_558"}
// Dependencies: {}
impl Regions { # [doc = " Returns true if none of this structure's tables contain any regions."] pub (crate) fn has_no_regions (& self) -> bool { let Self { code_regions , expansion_regions , branch_regions } = self ; code_regions . is_empty () && expansion_regions . is_empty () && branch_regions . is_empty () } }
};
}
