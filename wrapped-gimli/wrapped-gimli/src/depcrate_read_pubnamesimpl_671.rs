// Generated macro for impl_671 (impl)
macro_rules! Depcrate_read_pubnamesimpl_671 {
() => {
// Module: crate::read::pubnames
// Provides: {"impl_671"}
// Dependencies: {}
impl < R : Reader > Section < R > for DebugPubNames < R > { fn id () -> SectionId { SectionId :: DebugPubNames } fn reader (& self) -> & R { self . 0 . reader () } }
};
}
