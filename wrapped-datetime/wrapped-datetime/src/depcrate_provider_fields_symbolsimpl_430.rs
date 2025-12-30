// Generated macro for impl_430 (impl)
macro_rules! Depcrate_provider_fields_symbolsimpl_430 {
() => {
// Module: crate::provider::fields::symbols
// Provides: {"impl_430"}
// Dependencies: {}
impl DecimalSecond { # [inline] pub (crate) fn idx (self) -> u8 { self as u8 } # [inline] pub (crate) fn from_idx (idx : u8) -> Result < Self , SymbolError > { Self :: new_from_u8 (idx) . ok_or (SymbolError :: InvalidIndex (idx)) } }
};
}
