// Generated macro for impl_316 (impl)
macro_rules! Depcrate_read_dwarfimpl_316 {
() => {
// Module: crate::read::dwarf
// Provides: {"impl_316"}
// Dependencies: {}
impl < 'a , R : Reader > core :: ops :: Deref for Unit < R > { type Target = UnitHeader < R > ; fn deref (& self) -> & Self :: Target { & self . header } }
};
}
