// Generated macro for impl_850 (impl)
macro_rules! Depcrate_read_macho_sectionimpl_850 {
() => {
// Module: crate::read::macho::section
// Provides: {"impl_850"}
// Dependencies: {}
impl < Endian : endian :: Endian > Section for macho :: Section64 < Endian > { type Word = u64 ; type Endian = Endian ; fn sectname (& self) -> & [u8 ; 16] { & self . sectname } fn segname (& self) -> & [u8 ; 16] { & self . segname } fn addr (& self , endian : Self :: Endian) -> Self :: Word { self . addr . get (endian) } fn size (& self , endian : Self :: Endian) -> Self :: Word { self . size . get (endian) } fn offset (& self , endian : Self :: Endian) -> u32 { self . offset . get (endian) } fn align (& self , endian : Self :: Endian) -> u32 { self . align . get (endian) } fn reloff (& self , endian : Self :: Endian) -> u32 { self . reloff . get (endian) } fn nreloc (& self , endian : Self :: Endian) -> u32 { self . nreloc . get (endian) } fn flags (& self , endian : Self :: Endian) -> u32 { self . flags . get (endian) } }
};
}
