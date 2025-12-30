// Generated macro for impl_486 (impl)
macro_rules! Depcrate_read_indeximpl_486 {
() => {
// Module: crate::read::index
// Provides: {"impl_486"}
// Dependencies: {}
impl < R : Reader > DebugCuIndex < R > { # [doc = " Parse the index header."] pub fn index (self) -> Result < UnitIndex < R > > { UnitIndex :: parse (self . section) } }
};
}
