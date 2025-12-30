// Generated macro for Error (struct)
macro_rules! Depcrate_errorsError {
() => {
// Module: crate::errors
// Provides: {"Error"}
// Dependencies: {}
# [derive (Debug)] pub struct Error { pub line_num : Option < usize > , pub column_num : Option < usize > , # [doc = " What kind of message we expect (e.g., warning, error, suggestion)."] pub kind : ErrorKind , pub msg : String , # [doc = " For some `Error`s, like secondary lines of multi-line diagnostics, line annotations"] # [doc = " are not mandatory, even if they would otherwise be mandatory for primary errors."] # [doc = " Only makes sense for \"actual\" errors, not for \"expected\" errors."] pub require_annotation : bool , }
};
}
