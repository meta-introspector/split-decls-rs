// Generated macro for get_output_type (function)
macro_rules! Depcrateget_output_type {
() => {
// Module: crate
// Provides: {"get_output_type"}
// Dependencies: {}
# [doc = " Gets the file type to use for the output."] # [doc = " If the type arg is supplied, this is used."] # [doc = " Otherwise, the output type falls back to JSON as the default."] fn get_output_type (arg : Option < OutputType >) -> OutputType { if let Some (output_type) = arg { return output_type ; } OutputType :: Json }
};
}
