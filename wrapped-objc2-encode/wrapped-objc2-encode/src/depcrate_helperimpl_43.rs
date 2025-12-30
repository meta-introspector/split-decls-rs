// Generated macro for impl_43 (impl)
macro_rules! Depcrate_helperimpl_43 {
() => {
// Module: crate::helper
// Provides: {"impl_43"}
// Dependencies: {}
impl IndirectionKind { pub (crate) const fn prefix (self) -> char { self . prefix_byte () as char } pub (crate) const fn prefix_byte (self) -> u8 { match self { Self :: Atomic => b'A' , Self :: Pointer => b'^' , } } }
};
}
