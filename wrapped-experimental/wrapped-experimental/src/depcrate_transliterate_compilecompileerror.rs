// Generated macro for CompileError (struct)
macro_rules! Depcrate_transliterate_compileCompileError {
() => {
// Module: crate::transliterate::compile
// Provides: {"CompileError"}
// Dependencies: {}
struct CompileError { # [doc = " offset is the index to an arbitrary byte in the last character in the source that makes sense"] # [doc = " to display as location for the error, e.g., the unexpected character itself or"] # [doc = " for an unknown property name the last character of the name."] offset : Option < usize > , # [doc = " The type of compile error"] kind : CompileErrorKind , }
};
}
