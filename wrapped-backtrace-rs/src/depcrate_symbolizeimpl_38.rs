// Generated macro for impl_38 (impl)
macro_rules! Depcrate_symbolizeimpl_38 {
() => {
// Module: crate::symbolize
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'a > fmt :: Display for SymbolName < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (ref s) = self . demangled { return s . fmt (f) ; } # [cfg (feature = "cpp_demangle")] { if let Some (ref cpp) = self . cpp_demangled . 0 { if let Ok (s) = cpp . demangle () { return s . fmt (f) ; } } } format_symbol_name (fmt :: Display :: fmt , self . bytes , f) } }
};
}
