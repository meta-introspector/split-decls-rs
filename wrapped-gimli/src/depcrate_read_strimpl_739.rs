// Generated macro for impl_739 (impl)
macro_rules! Depcrate_read_strimpl_739 {
() => {
// Module: crate::read::str
// Provides: {"impl_739"}
// Dependencies: {}
impl < R > Section < R > for DebugStr < R > { fn id () -> SectionId { SectionId :: DebugStr } fn reader (& self) -> & R { & self . debug_str_section } }
};
}
