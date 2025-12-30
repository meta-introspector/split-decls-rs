// Generated macro for validate_label (function)
macro_rules! Depcrate_grammarvalidate_label {
() => {
// Module: crate::grammar
// Provides: {"validate_label"}
// Dependencies: {}
# [doc = " Validate that the given bytes are allowed as a PEM type label, i.e. the"] # [doc = " label encoded in the `BEGIN` and `END` encapsulation boundaries."] pub (crate) fn validate_label (label : & [u8]) -> Result < () > { let mut last_was_wsp = false ; for & char in label { if ! is_allowed_in_label (char) { return Err (Error :: Label) ; } if is_wsp (char) { if last_was_wsp { return Err (Error :: Label) ; } last_was_wsp = true ; } else { last_was_wsp = false ; } } Ok (()) }
};
}
