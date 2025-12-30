// Generated macro for impl_672 (impl)
macro_rules! Depcrate_read_pubnamesimpl_672 {
() => {
// Module: crate::read::pubnames
// Provides: {"impl_672"}
// Dependencies: {}
impl < R : Reader > From < R > for DebugPubNames < R > { fn from (debug_pubnames_section : R) -> Self { DebugPubNames (DebugLookup :: from (debug_pubnames_section)) } }
};
}
