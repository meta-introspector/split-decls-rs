// Generated macro for impl_464 (impl)
macro_rules! Depcrate_provider_fieldsimpl_464 {
() => {
// Module: crate::provider::fields
// Provides: {"impl_464"}
// Dependencies: {}
impl FieldULE { # [inline] pub (crate) fn validate_byte_pair (bytes : (u8 , u8)) -> Result < () , zerovec :: ule :: UleError > { symbols :: FieldSymbolULE :: validate_byte (bytes . 0) ? ; length :: FieldLengthULE :: validate_byte (bytes . 1) ? ; Ok (()) } }
};
}
