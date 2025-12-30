// Generated macro for impl_492 (impl)
macro_rules! Depcrate_read_indeximpl_492 {
() => {
// Module: crate::read::index
// Provides: {"impl_492"}
// Dependencies: {}
impl < R : Reader > DebugTuIndex < R > { # [doc = " Parse the index header."] pub fn index (self) -> Result < UnitIndex < R > > { UnitIndex :: parse (self . section) } }
};
}
