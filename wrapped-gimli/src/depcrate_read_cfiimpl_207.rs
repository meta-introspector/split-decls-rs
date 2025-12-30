// Generated macro for impl_207 (impl)
macro_rules! Depcrate_read_cfiimpl_207 {
() => {
// Module: crate::read::cfi
// Provides: {"impl_207"}
// Dependencies: {}
impl < R : Reader > Section < R > for EhFrameHdr < R > { fn id () -> SectionId { SectionId :: EhFrameHdr } fn reader (& self) -> & R { & self . 0 } }
};
}
