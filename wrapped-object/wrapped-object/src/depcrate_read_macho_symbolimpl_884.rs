// Generated macro for impl_884 (impl)
macro_rules! Depcrate_read_macho_symbolimpl_884 {
() => {
// Module: crate::read::macho::symbol
// Provides: {"impl_884"}
// Dependencies: {}
impl < Endian : endian :: Endian > Nlist for macho :: Nlist64 < Endian > { type Word = u64 ; type Endian = Endian ; fn n_strx (& self , endian : Self :: Endian) -> u32 { self . n_strx . get (endian) } fn n_type (& self) -> u8 { self . n_type } fn n_sect (& self) -> u8 { self . n_sect } fn n_desc (& self , endian : Self :: Endian) -> u16 { self . n_desc . get (endian) } fn n_value (& self , endian : Self :: Endian) -> Self :: Word { self . n_value . get (endian) } }
};
}
