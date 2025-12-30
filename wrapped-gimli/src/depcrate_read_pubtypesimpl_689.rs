// Generated macro for impl_689 (impl)
macro_rules! Depcrate_read_pubtypesimpl_689 {
() => {
// Module: crate::read::pubtypes
// Provides: {"impl_689"}
// Dependencies: {}
impl < R : Reader > From < R > for DebugPubTypes < R > { fn from (debug_pubtypes_section : R) -> Self { DebugPubTypes (DebugLookup :: from (debug_pubtypes_section)) } }
};
}
