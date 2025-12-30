// Generated macro for impl_745 (impl)
macro_rules! Depcrate_read_macho_fatimpl_745 {
() => {
// Module: crate::read::macho::fat
// Provides: {"impl_745"}
// Dependencies: {}
impl FatArch for FatArch64 { type Word = u64 ; const MAGIC : u32 = macho :: FAT_MAGIC_64 ; fn cputype (& self) -> u32 { self . cputype . get (BigEndian) } fn cpusubtype (& self) -> u32 { self . cpusubtype . get (BigEndian) } fn offset (& self) -> Self :: Word { self . offset . get (BigEndian) } fn size (& self) -> Self :: Word { self . size . get (BigEndian) } fn align (& self) -> u32 { self . align . get (BigEndian) } }
};
}
