// Generated macro for impl_422 (impl)
macro_rules! Depcrate_provider_fields_symbolsimpl_422 {
() => {
// Module: crate::provider::fields::symbols
// Provides: {"impl_422"}
// Dependencies: {}
impl Week { # [doc = " Retrieves an index of the field variant."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " use icu::datetime::fields::Month;"] # [doc = ""] # [doc = " assert_eq!(Month::StandAlone::idx(), 1);"] # [doc = " ```"] # [doc = ""] # [doc = " # Stability"] # [doc = ""] # [doc = " This is mostly useful for serialization,"] # [doc = " and does not guarantee index stability between ICU4X"] # [doc = " versions."] # [inline] pub (crate) fn idx (self) -> u8 { 0 } # [doc = " Retrieves a field variant from an index."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```ignore"] # [doc = " use icu::datetime::fields::Month;"] # [doc = ""] # [doc = " assert_eq!(Month::from_idx(0), Month::Format);"] # [doc = " ```"] # [doc = ""] # [doc = " # Stability"] # [doc = ""] # [doc = " This is mostly useful for serialization,"] # [doc = " and does not guarantee index stability between ICU4X"] # [doc = " versions."] # [inline] pub (crate) fn from_idx (idx : u8) -> Result < Self , SymbolError > { Err (SymbolError :: InvalidIndex (idx)) } }
};
}
