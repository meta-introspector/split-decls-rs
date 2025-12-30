// Generated macro for impl_383 (impl)
macro_rules! Depcrate_provider_fields_lengthimpl_383 {
() => {
// Module: crate::provider::fields::length
// Provides: {"impl_383"}
// Dependencies: {}
impl FieldLengthULE { # [inline] pub (crate) fn validate_byte (byte : u8) -> Result < () , UleError > { FieldLength :: from_idx (byte) . map (| _ | ()) . map_err (| _ | UleError :: parse :: < FieldLength > ()) } }
};
}
