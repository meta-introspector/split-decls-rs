// Generated macro for Symbol (struct)
macro_rules! Depcrate_writeSymbol {
() => {
// Module: crate::write
// Provides: {"Symbol"}
// Dependencies: {}
# [doc = " A symbol in an object file."] # [derive (Debug)] pub struct Symbol { # [doc = " The name of the symbol."] pub name : Vec < u8 > , # [doc = " The value of the symbol."] # [doc = ""] # [doc = " If the symbol defined in a section, then this is the section offset of the symbol."] pub value : u64 , # [doc = " The size of the symbol."] pub size : u64 , # [doc = " The kind of the symbol."] pub kind : SymbolKind , # [doc = " The scope of the symbol."] pub scope : SymbolScope , # [doc = " Whether the symbol has weak binding."] pub weak : bool , # [doc = " The section containing the symbol."] pub section : SymbolSection , # [doc = " Symbol flags that are specific to each file format."] pub flags : SymbolFlags < SectionId , SymbolId > , }
};
}
