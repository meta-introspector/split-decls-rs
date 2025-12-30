// Generated macro for impl_45 (impl)
macro_rules! Depcrate_helperimpl_45 {
() => {
// Module: crate::helper
// Provides: {"impl_45"}
// Dependencies: {}
impl ContainerKind { pub (crate) const fn start (self) -> char { self . start_byte () as char } pub (crate) const fn end (self) -> char { self . end_byte () as char } pub (crate) const fn start_byte (self) -> u8 { match self { Self :: Struct => b'{' , Self :: Union => b'(' , } } pub (crate) const fn end_byte (self) -> u8 { match self { Self :: Struct => b'}' , Self :: Union => b')' , } } }
};
}
