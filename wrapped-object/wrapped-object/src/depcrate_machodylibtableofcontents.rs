// Generated macro for DylibTableOfContents (struct)
macro_rules! Depcrate_machoDylibTableOfContents {
() => {
// Module: crate::macho
// Provides: {"DylibTableOfContents"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct DylibTableOfContents < E : Endian > { # [doc = " the defined external symbol (index into the symbol table)"] pub symbol_index : U32 < E > , # [doc = " index into the module table this symbol is defined in"] pub module_index : U32 < E > , }
};
}
