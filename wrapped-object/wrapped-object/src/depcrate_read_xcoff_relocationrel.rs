// Generated macro for Rel (trait)
macro_rules! Depcrate_read_xcoff_relocationRel {
() => {
// Module: crate::read::xcoff::relocation
// Provides: {"Rel"}
// Dependencies: {}
# [doc = " A trait for generic access to [`xcoff::Rel32`] and [`xcoff::Rel64`]."] # [allow (missing_docs)] pub trait Rel : Debug + Pod { type Word : Into < u64 > ; fn r_vaddr (& self) -> Self :: Word ; fn r_symndx (& self) -> u32 ; fn r_rsize (& self) -> u8 ; fn r_rtype (& self) -> u8 ; fn symbol (& self) -> SymbolIndex { SymbolIndex (self . r_symndx () as usize) } }
};
}
