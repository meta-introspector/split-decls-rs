// Generated macro for replace_cargo (function)
macro_rules! Depcrate_testsreplace_cargo {
() => {
// Module: crate::tests
// Provides: {"replace_cargo"}
// Dependencies: {}
fn replace_cargo (s : & mut String) { let path = toolchain :: Tool :: Cargo . path () . to_string () . escape_debug () . collect :: < String > () ; * s = s . replace (& path , "$CARGO$") ; }
};
}
