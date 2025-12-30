// Generated macro for Rel (trait)
macro_rules! Depcrate_read_elf_relocationRel {
() => {
// Module: crate::read::elf::relocation
// Provides: {"Rel"}
// Dependencies: {}
# [doc = " A trait for generic access to [`elf::Rel32`] and [`elf::Rel64`]."] # [allow (missing_docs)] pub trait Rel : Debug + Pod + Clone { type Word : Into < u64 > ; type Sword : Into < i64 > ; type Endian : endian :: Endian ; fn r_offset (& self , endian : Self :: Endian) -> Self :: Word ; fn r_info (& self , endian : Self :: Endian) -> Self :: Word ; fn r_sym (& self , endian : Self :: Endian) -> u32 ; fn r_type (& self , endian : Self :: Endian) -> u32 ; # [doc = " Get the symbol index referenced by the relocation."] # [doc = ""] # [doc = " Returns `None` for the null symbol index."] fn symbol (& self , endian : Self :: Endian) -> Option < SymbolIndex > { let sym = self . r_sym (endian) ; if sym == 0 { None } else { Some (SymbolIndex (sym as usize)) } } }
};
}
