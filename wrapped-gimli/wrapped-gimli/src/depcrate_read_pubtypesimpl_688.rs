// Generated macro for impl_688 (impl)
macro_rules! Depcrate_read_pubtypesimpl_688 {
() => {
// Module: crate::read::pubtypes
// Provides: {"impl_688"}
// Dependencies: {}
impl < R : Reader > Section < R > for DebugPubTypes < R > { fn id () -> SectionId { SectionId :: DebugPubTypes } fn reader (& self) -> & R { self . 0 . reader () } }
};
}
