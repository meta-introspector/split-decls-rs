// Generated macro for impl_18 (impl)
macro_rules! Depcrate_elf2table_symbolimpl_18 {
() => {
// Module: crate::elf2table::symbol
// Provides: {"impl_18"}
// Dependencies: {}
impl Symbol { pub fn demangle (raw : & str) -> anyhow :: Result < Self > { serde_json :: from_str (raw) . map_err (| j | anyhow :: anyhow ! ("failed to demangle defmt symbol `{}`: {}" , raw , j)) } pub fn tag (& self) -> SymbolTag { match & * self . tag { "defmt_prim" => SymbolTag :: Defmt (Tag :: Prim) , "defmt_derived" => SymbolTag :: Defmt (Tag :: Derived) , "defmt_bitflags" => SymbolTag :: Defmt (Tag :: Bitflags) , "defmt_write" => SymbolTag :: Defmt (Tag :: Write) , "defmt_timestamp" => SymbolTag :: Defmt (Tag :: Timestamp) , "defmt_bitflags_value" => SymbolTag :: Defmt (Tag :: BitflagsValue) , "defmt_str" => SymbolTag :: Defmt (Tag :: Str) , "defmt_println" => SymbolTag :: Defmt (Tag :: Println) , "defmt_trace" => SymbolTag :: Defmt (Tag :: Trace) , "defmt_debug" => SymbolTag :: Defmt (Tag :: Debug) , "defmt_info" => SymbolTag :: Defmt (Tag :: Info) , "defmt_warn" => SymbolTag :: Defmt (Tag :: Warn) , "defmt_error" => SymbolTag :: Defmt (Tag :: Error) , _ => SymbolTag :: Custom (()) , } } pub fn data (& self) -> & str { & self . data } pub fn package (& self) -> & str { & self . package } pub fn disambiguator (& self) -> & str { & self . disambiguator } pub fn crate_name (& self) -> Option < & str > { self . crate_name . as_deref () } }
};
}
