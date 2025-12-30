// Generated macro for impl_1425 (impl)
macro_rules! Depcrate_write_machoimpl_1425 {
() => {
// Module: crate::write::macho
// Provides: {"impl_1425"}
// Dependencies: {}
impl MachOBuildVersion { fn cmdsize (& self) -> u32 { let sz = mem :: size_of :: < macho :: BuildVersionCommand < Endianness > > () ; debug_assert ! (sz <= u32 :: MAX as usize) ; sz as u32 } }
};
}
