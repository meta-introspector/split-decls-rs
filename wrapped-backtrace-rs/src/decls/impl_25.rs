macro_rules! deps {
    () => {
        SymbolName!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < 'a > fmt :: Display for SymbolName < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if let Some (ref s) = self . demangled { return s . fmt (f) ; } # [cfg (feature = "cpp_demangle")] { if let Some (ref cpp) = self . cpp_demangled . 0 { if let Ok (s) = cpp . demangle () { return s . fmt (f) ; } } } format_symbol_name (fmt :: Display :: fmt , self . bytes , f) } }
    };
}

impl_25!()