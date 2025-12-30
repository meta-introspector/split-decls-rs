// Generated macro for impl_777 (impl)
macro_rules! Depcrate_read_macho_fileimpl_777 {
() => {
// Module: crate::read::macho::file
// Provides: {"impl_777"}
// Dependencies: {}
impl < Endian : endian :: Endian > MachHeader for macho :: MachHeader64 < Endian > { type Word = u64 ; type Endian = Endian ; type Segment = macho :: SegmentCommand64 < Endian > ; type Section = macho :: Section64 < Endian > ; type Nlist = macho :: Nlist64 < Endian > ; fn is_type_64 (& self) -> bool { true } fn is_big_endian (& self) -> bool { self . magic () == macho :: MH_MAGIC_64 } fn is_little_endian (& self) -> bool { self . magic () == macho :: MH_CIGAM_64 } fn magic (& self) -> u32 { self . magic . get (BigEndian) } fn cputype (& self , endian : Self :: Endian) -> u32 { self . cputype . get (endian) } fn cpusubtype (& self , endian : Self :: Endian) -> u32 { self . cpusubtype . get (endian) } fn filetype (& self , endian : Self :: Endian) -> u32 { self . filetype . get (endian) } fn ncmds (& self , endian : Self :: Endian) -> u32 { self . ncmds . get (endian) } fn sizeofcmds (& self , endian : Self :: Endian) -> u32 { self . sizeofcmds . get (endian) } fn flags (& self , endian : Self :: Endian) -> u32 { self . flags . get (endian) } }
};
}
