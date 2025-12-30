// Generated macro for KnownSymbol (enum)
macro_rules! Depcrate_ir_known_symbolKnownSymbol {
() => {
// Module: crate::ir::known_symbol
// Provides: {"KnownSymbol"}
// Dependencies: {}
# [doc = " A well-known symbol."] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] # [cfg_attr (feature = "enable-serde" , derive (Serialize , Deserialize))] pub enum KnownSymbol { # [doc = " ELF well-known linker symbol _GLOBAL_OFFSET_TABLE_"] ElfGlobalOffsetTable , # [doc = " TLS index symbol for the current thread."] # [doc = " Used in COFF/PE file formats."] CoffTlsIndex , }
};
}
