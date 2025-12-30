// Generated macro for is_allowed_in_label (function)
macro_rules! Depcrate_grammaris_allowed_in_label {
() => {
// Module: crate::grammar
// Provides: {"is_allowed_in_label"}
// Dependencies: {}
# [doc = " Does the provided byte match a character allowed in a label?"] pub (crate) fn is_allowed_in_label (char : u8) -> bool { is_labelchar (char) || matches ! (char , CHAR_HT | CHAR_SP) }
};
}
