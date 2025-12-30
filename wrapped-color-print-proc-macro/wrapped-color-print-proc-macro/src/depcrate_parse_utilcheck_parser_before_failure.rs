// Generated macro for check_parser_before_failure (function)
macro_rules! Depcrate_parse_utilcheck_parser_before_failure {
() => {
// Module: crate::parse::util
// Provides: {"check_parser_before_failure"}
// Dependencies: {}
# [doc = " Checks if the first parser succeeds, then parses the input with the second parser. If an error"] # [doc = " is encountered with the second parser, then a failure message is thrown."] pub fn check_parser_before_failure < 'a , C , CV , P , PV > (mut check_parser : C , mut parser : P , failure_msg : & 'a str) -> impl Parser < 'a , PV > where C : Parser < 'a , CV > , P : Parser < 'a , PV > , { move | input | { check_parser (input) ? ; with_failure_message (| input | { parser (input) } , failure_msg) (input) } }
};
}
