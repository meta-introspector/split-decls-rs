// Generated macro for impl_80 (impl)
macro_rules! Depcrate_typesimpl_80 {
() => {
// Module: crate::types
// Provides: {"impl_80"}
// Dependencies: {}
impl < 'de , const REPLACEMENT_ALLOWED : bool > Deserialize < 'de > for DisallowedPath < REPLACEMENT_ALLOWED > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let enum_ = DisallowedPathEnum :: deserialize (deserializer) ? ; if ! REPLACEMENT_ALLOWED && enum_ . replacement () . is_some () { return Err (de :: Error :: custom ("replacement not allowed for this configuration")) ; } Ok (Self { path : enum_ . path () . to_owned () , reason : enum_ . reason () . map (ToOwned :: to_owned) , replacement : enum_ . replacement () . map (ToOwned :: to_owned) , allow_invalid : enum_ . allow_invalid () , span : Span :: default () , }) } }
};
}
