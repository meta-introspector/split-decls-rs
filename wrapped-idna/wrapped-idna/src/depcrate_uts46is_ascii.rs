// Generated macro for is_ascii (function)
macro_rules! Depcrate_uts46is_ascii {
() => {
// Module: crate::uts46
// Provides: {"is_ascii"}
// Dependencies: {}
# [inline (always)] fn is_ascii (label : & [char]) -> bool { for c in label . iter () { if ! c . is_ascii () { return false ; } } true }
};
}
