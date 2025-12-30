// Generated macro for impl_520 (impl)
macro_rules! Depcrate_read_elf_relocationimpl_520 {
() => {
// Module: crate::read::elf::relocation
// Provides: {"impl_520"}
// Dependencies: {}
impl < Endian : endian :: Endian > Rela for elf :: Rela64 < Endian > { type Word = u64 ; type Sword = i64 ; type Endian = Endian ; # [inline] fn r_offset (& self , endian : Self :: Endian) -> Self :: Word { self . r_offset . get (endian) } # [inline] fn r_info (& self , endian : Self :: Endian , is_mips64el : bool) -> Self :: Word { self . get_r_info (endian , is_mips64el) } # [inline] fn r_addend (& self , endian : Self :: Endian) -> Self :: Sword { self . r_addend . get (endian) } # [inline] fn r_sym (& self , endian : Self :: Endian , is_mips64el : bool) -> u32 { self . r_sym (endian , is_mips64el) } # [inline] fn r_type (& self , endian : Self :: Endian , is_mips64el : bool) -> u32 { self . r_type (endian , is_mips64el) } }
};
}
