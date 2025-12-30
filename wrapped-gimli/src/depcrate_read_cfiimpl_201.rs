// Generated macro for impl_201 (impl)
macro_rules! Depcrate_read_cfiimpl_201 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_201"}
// Dependencies: {}
impl < R : Reader > Section < R > for DebugFrame < R > { fn id () -> SectionId { SectionId :: DebugFrame } fn reader (& self) -> & R { & self . section } }
};
}
