// Generated macro for impl_487 (impl)
macro_rules! Depcrate_read_elf_symbolimpl_487 {
() => {
// Module: crate::read::elf::symbol
// Provides: {"impl_487"}
// Dependencies: {}
impl < Endian : endian :: Endian > Sym for elf :: Sym64 < Endian > { type Word = u64 ; type Endian = Endian ; # [inline] fn st_name (& self , endian : Self :: Endian) -> u32 { self . st_name . get (endian) } # [inline] fn st_info (& self) -> u8 { self . st_info } # [inline] fn st_bind (& self) -> u8 { self . st_bind () } # [inline] fn st_type (& self) -> u8 { self . st_type () } # [inline] fn st_other (& self) -> u8 { self . st_other } # [inline] fn st_visibility (& self) -> u8 { self . st_visibility () } # [inline] fn st_shndx (& self , endian : Self :: Endian) -> u16 { self . st_shndx . get (endian) } # [inline] fn st_value (& self , endian : Self :: Endian) -> Self :: Word { self . st_value . get (endian) } # [inline] fn st_size (& self , endian : Self :: Endian) -> Self :: Word { self . st_size . get (endian) } }
};
}
