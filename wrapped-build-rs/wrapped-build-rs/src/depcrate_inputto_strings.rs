// Generated macro for to_strings (function)
macro_rules! Depcrate_inputto_strings {
() => {
// Module: crate::input
// Provides: {"to_strings"}
// Dependencies: {}
# [track_caller] fn to_strings (value : std :: ffi :: OsString , sep : char) -> Vec < String > { if value . is_empty () { return Vec :: new () ; } let value = to_string (value) ; value . split (sep) . map (str :: to_owned) . collect () }
};
}
