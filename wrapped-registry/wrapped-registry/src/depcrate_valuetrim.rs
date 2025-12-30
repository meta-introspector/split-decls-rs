// Generated macro for trim (function)
macro_rules! Depcrate_valuetrim {
() => {
// Module: crate::value
// Provides: {"trim"}
// Dependencies: {}
fn trim (mut wide : & [u16]) -> & [u16] { while wide . last () == Some (& 0) { wide = & wide [.. wide . len () - 1] ; } wide }
};
}
