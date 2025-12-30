// Generated macro for SymbolName (struct)
macro_rules! Depcrate_symbolizeSymbolName {
() => {
// Module: crate::symbolize
// Provides: {"SymbolName"}
// Dependencies: {}
# [doc = " A wrapper around a symbol name to provide ergonomic accessors to the"] # [doc = " demangled name, the raw bytes, the raw string, etc."] pub struct SymbolName < 'a > { bytes : & 'a [u8] , demangled : Option < Demangle < 'a > > , # [cfg (feature = "cpp_demangle")] cpp_demangled : OptionCppSymbol < 'a > , }
};
}
