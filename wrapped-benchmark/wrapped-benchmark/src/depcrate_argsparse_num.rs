// Generated macro for parse_num (function)
macro_rules! Depcrate_argsparse_num {
() => {
// Module: crate::args
// Provides: {"parse_num"}
// Dependencies: {}
fn parse_num (names : & [& str] , name : & str , value : & str) -> usize { value . parse () . unwrap_or_else (| _ | print_usage (names , Some (format ! ("Invalid value for {name}: {value}")))) }
};
}
