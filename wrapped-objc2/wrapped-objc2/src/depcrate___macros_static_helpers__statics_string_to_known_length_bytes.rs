// Generated macro for __statics_string_to_known_length_bytes (macro)
macro_rules! Depcrate___macros_static_helpers__statics_string_to_known_length_bytes {
() => {
// Module: crate::__macros::static_helpers
// Provides: {"__statics_string_to_known_length_bytes"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __statics_string_to_known_length_bytes { ($ inp : ident) => { { let mut res : [$ crate :: __macros :: u8 ; $ inp . len ()] = [0 ; $ inp . len ()] ; let mut i = 0 ; while i < $ inp . len () { res [i] = $ inp [i] ; i += 1 ; } res } } ; }
};
}
