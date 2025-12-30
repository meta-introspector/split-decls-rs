// Generated macro for impl_39 (impl)
macro_rules! Depcrate_symbolizeimpl_39 {
() => {
// Module: crate::symbolize
// Provides: {"impl_39"}
// Dependencies: {}
impl < 'a > fmt :: Debug for SymbolName < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (ref s) = self . demangled { return s . fmt (f) ; } # [cfg (all (feature = "std" , feature = "cpp_demangle"))] { if let Some (ref cpp) = self . cpp_demangled . 0 { if let Ok (s) = cpp . demangle () { return s . fmt (f) ; } } } format_symbol_name (fmt :: Debug :: fmt , self . bytes , f) } }
};
}
