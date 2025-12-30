// Generated macro for is_outer (function)
macro_rules! Depcrate_shortcutsis_outer {
() => {
// Module: crate::shortcuts
// Provides: {"is_outer"}
// Dependencies: {}
fn is_outer (text : & str) -> bool { if text . starts_with ("////") || text . starts_with ("/***") { return false ; } text . starts_with ("///") || text . starts_with ("/**") }
};
}
