// Generated macro for Relocation (struct)
macro_rules! Depcrate_writeRelocation {
() => {
// Module: crate::write
// Provides: {"Relocation"}
// Dependencies: {}
# [doc = " A relocation in an object file."] # [derive (Debug)] pub struct Relocation { # [doc = " The section offset of the place of the relocation."] pub offset : u64 , # [doc = " The symbol referred to by the relocation."] # [doc = ""] # [doc = " This may be a section symbol."] pub symbol : SymbolId , # [doc = " The addend to use in the relocation calculation."] # [doc = ""] # [doc = " This may be in addition to an implicit addend stored at the place of the relocation."] pub addend : i64 , # [doc = " The fields that define the relocation type."] pub flags : RelocationFlags , }
};
}
