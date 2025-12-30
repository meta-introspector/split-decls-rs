// Generated macro for impl_516 (impl)
macro_rules! Depcrate_read_elf_relocationimpl_516 {
() => {
// Module: crate::read::elf::relocation
// Provides: {"impl_516"}
// Dependencies: {}
impl < Endian : endian :: Endian > Rel for elf :: Rel32 < Endian > { type Word = u32 ; type Sword = i32 ; type Endian = Endian ; # [inline] fn r_offset (& self , endian : Self :: Endian) -> Self :: Word { self . r_offset . get (endian) } # [inline] fn r_info (& self , endian : Self :: Endian) -> Self :: Word { self . r_info . get (endian) } # [inline] fn r_sym (& self , endian : Self :: Endian) -> u32 { self . r_sym (endian) } # [inline] fn r_type (& self , endian : Self :: Endian) -> u32 { self . r_type (endian) } }
};
}
