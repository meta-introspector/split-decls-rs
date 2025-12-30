// Generated macro for impl_497 (impl)
macro_rules! Depcrate_read_indeximpl_497 {
() => {
// Module: crate::read::index
// Provides: {"impl_497"}
// Dependencies: {}
impl < 'index , R : Reader > Iterator for UnitIndexSectionIterator < 'index , R > { type Item = UnitIndexSection ; fn next (& mut self) -> Option < UnitIndexSection > { let section = * self . sections . next () ? ; let offset = self . offsets . read_u32 () . ok () ? ; let size = self . sizes . read_u32 () . ok () ? ; Some (UnitIndexSection { section , offset , size , }) } }
};
}
