// Generated macro for macro_536 (macro)
macro_rules! Depcrate_encodemacro_536 {
() => {
// Module: crate::encode
// Provides: {"macro_536"}
// Dependencies: {}
encode_pointer_impls ! (unsafe impl < T : RefEncode > RefEncode for Pointer < T > { const ENCODING_REF = Encoding :: Pointer (& T :: ENCODING_REF) ; }) ;
};
}
