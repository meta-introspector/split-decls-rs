// Generated macro for impl_218 (impl)
macro_rules! Depcrate_read_cfiimpl_218 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_218"}
// Dependencies: {}
impl < R : Reader > Section < R > for EhFrame < R > { fn id () -> SectionId { SectionId :: EhFrame } fn reader (& self) -> & R { & self . section } }
};
}
