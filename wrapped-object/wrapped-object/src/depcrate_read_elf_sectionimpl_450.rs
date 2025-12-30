// Generated macro for impl_450 (impl)
macro_rules! Depcrate_read_elf_sectionimpl_450 {
() => {
// Module: crate::read::elf::section
// Provides: {"impl_450"}
// Dependencies: {}
impl < Endian : endian :: Endian > SectionHeader for elf :: SectionHeader32 < Endian > { type Elf = elf :: FileHeader32 < Endian > ; type Word = u32 ; type Endian = Endian ; # [inline] fn sh_name (& self , endian : Self :: Endian) -> u32 { self . sh_name . get (endian) } # [inline] fn sh_type (& self , endian : Self :: Endian) -> u32 { self . sh_type . get (endian) } # [inline] fn sh_flags (& self , endian : Self :: Endian) -> Self :: Word { self . sh_flags . get (endian) } # [inline] fn sh_addr (& self , endian : Self :: Endian) -> Self :: Word { self . sh_addr . get (endian) } # [inline] fn sh_offset (& self , endian : Self :: Endian) -> Self :: Word { self . sh_offset . get (endian) } # [inline] fn sh_size (& self , endian : Self :: Endian) -> Self :: Word { self . sh_size . get (endian) } # [inline] fn sh_link (& self , endian : Self :: Endian) -> u32 { self . sh_link . get (endian) } # [inline] fn sh_info (& self , endian : Self :: Endian) -> u32 { self . sh_info . get (endian) } # [inline] fn sh_addralign (& self , endian : Self :: Endian) -> Self :: Word { self . sh_addralign . get (endian) } # [inline] fn sh_entsize (& self , endian : Self :: Endian) -> Self :: Word { self . sh_entsize . get (endian) } }
};
}
