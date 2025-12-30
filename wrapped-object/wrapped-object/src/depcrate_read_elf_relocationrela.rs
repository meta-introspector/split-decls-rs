// Generated macro for Rela (trait)
macro_rules! Depcrate_read_elf_relocationRela {
() => {
// Module: crate::read::elf::relocation
// Provides: {"Rela"}
// Dependencies: {}
# [doc = " A trait for generic access to [`elf::Rela32`] and [`elf::Rela64`]."] # [allow (missing_docs)] pub trait Rela : Debug + Pod + Clone { type Word : Into < u64 > ; type Sword : Into < i64 > ; type Endian : endian :: Endian ; fn r_offset (& self , endian : Self :: Endian) -> Self :: Word ; fn r_info (& self , endian : Self :: Endian , is_mips64el : bool) -> Self :: Word ; fn r_addend (& self , endian : Self :: Endian) -> Self :: Sword ; fn r_sym (& self , endian : Self :: Endian , is_mips64el : bool) -> u32 ; fn r_type (& self , endian : Self :: Endian , is_mips64el : bool) -> u32 ; # [doc = " Get the symbol index referenced by the relocation."] # [doc = ""] # [doc = " Returns `None` for the null symbol index."] fn symbol (& self , endian : Self :: Endian , is_mips64el : bool) -> Option < SymbolIndex > { let sym = self . r_sym (endian , is_mips64el) ; if sym == 0 { None } else { Some (SymbolIndex (sym as usize)) } } }
};
}
