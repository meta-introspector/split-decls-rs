// Generated macro for impl_403 (impl)
macro_rules! Depcrate_provider_fields_symbolsimpl_403 {
() => {
// Module: crate::provider::fields::symbols
// Provides: {"impl_403"}
// Dependencies: {}
impl FieldSymbolULE { # [inline] pub (crate) fn validate_byte (byte : u8) -> Result < () , UleError > { FieldSymbol :: from_idx (byte) . map (| _ | ()) . map_err (| _ | UleError :: parse :: < FieldSymbol > ()) } }
};
}
