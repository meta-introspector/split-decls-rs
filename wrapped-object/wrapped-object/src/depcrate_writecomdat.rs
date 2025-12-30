// Generated macro for Comdat (struct)
macro_rules! Depcrate_writeComdat {
() => {
// Module: crate::write
// Provides: {"Comdat"}
// Dependencies: {}
# [doc = " A COMDAT section group."] # [derive (Debug)] pub struct Comdat { # [doc = " The COMDAT selection kind."] # [doc = ""] # [doc = " This determines the way in which the linker resolves multiple definitions of the COMDAT"] # [doc = " sections."] pub kind : ComdatKind , # [doc = " The COMDAT symbol."] # [doc = ""] # [doc = " If this symbol is referenced, then all sections in the group will be included by the"] # [doc = " linker."] pub symbol : SymbolId , # [doc = " The sections in the group."] pub sections : Vec < SectionId > , }
};
}
