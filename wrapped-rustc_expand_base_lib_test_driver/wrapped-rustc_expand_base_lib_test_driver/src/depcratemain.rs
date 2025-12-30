// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { println ! ("Testing rustc_expand_base_lib_errors and rustc_expand_base_lib_macros...") ; let dummy_span : Span = DummySpan . into () ; let _ = rustc_expand_base_lib_errors :: TraceMacroBase { span : dummy_span } ; println ! ("TraceMacroBase created successfully.") ; let _ = rustc_expand_base_lib_errors :: TraceMacroNote { span : dummy_span , message : "test message" . to_string () } ; println ! ("TraceMacroNote created successfully.") ; println ! ("Derive macros applied to test structs successfully (compile-time check).") ; }
};
}
