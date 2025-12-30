// Generated macro for line_should_be_ignored (function)
macro_rules! Depcrate_cargoline_should_be_ignored {
() => {
// Module: crate::cargo
// Provides: {"line_should_be_ignored"}
// Dependencies: {}
fn line_should_be_ignored (line : & str) -> bool { for check in IGNORED_LINES . iter () { if line . starts_with (check) { return true ; } } false }
};
}
