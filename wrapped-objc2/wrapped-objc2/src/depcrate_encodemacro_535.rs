// Generated macro for macro_535 (macro)
macro_rules! Depcrate_encodemacro_535 {
() => {
// Module: crate::encode
// Provides: {"macro_535"}
// Dependencies: {}
encode_pointer_impls ! (unsafe impl < T : RefEncode > Encode for Pointer < T > { const ENCODING = T :: ENCODING_REF ; }) ;
};
}
