// Generated macro for parse_output (function)
macro_rules! Depcrate_executor_jsonparse_output {
() => {
// Module: crate::executor::json
// Provides: {"parse_output"}
// Dependencies: {}
pub fn parse_output (file_name : & str , output : & str) -> Vec < Error > { let mut errors = Vec :: new () ; for line in output . lines () { match serde_json :: from_str :: < Diagnostic > (line) { Ok (diagnostic) => push_actual_errors (& mut errors , & diagnostic , & [] , file_name) , Err (_) => errors . push (Error { line_num : None , column_num : None , kind : ErrorKind :: Raw , msg : line . to_string () , require_annotation : false , }) , } } errors }
};
}
