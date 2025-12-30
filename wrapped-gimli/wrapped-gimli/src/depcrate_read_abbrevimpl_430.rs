// Generated macro for impl_430 (impl)
macro_rules! Depcrate_read_abbrevimpl_430 {
() => {
// Module: crate::read::abbrev
// Provides: {"impl_430"}
// Dependencies: {}
impl < R > Section < R > for DebugAbbrev < R > { fn id () -> SectionId { SectionId :: DebugAbbrev } fn reader (& self) -> & R { & self . debug_abbrev_section } }
};
}
