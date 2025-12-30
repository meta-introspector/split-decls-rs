// Generated macro for parse_one (function)
macro_rules! Depcrate_argsparse_one {
() => {
// Module: crate::args
// Provides: {"parse_one"}
// Dependencies: {}
fn parse_one (names : & [& str] , name : & str , value : & str) -> ArgRange { let components = value . split (':') . collect :: < Vec < _ > > () ; match components . len () { 1 => { let val = parse_num (names , name , components [0]) ; ArgRange { current : val , limit : val , step : 1 , } } 2 => { let start = parse_num (names , name , components [0]) ; let end = parse_num (names , name , components [1]) ; if start > end { print_usage (names , Some (format ! ("Invalid range for {name}: {value}"))) ; } ArgRange { current : start , limit : end , step : 1 , } } 3 => { let start = parse_num (names , name , components [0]) ; let end = parse_num (names , name , components [1]) ; let step = parse_num (names , name , components [2]) ; if start > end { print_usage (names , Some (format ! ("Invalid range for {name}: {value}"))) ; } ArgRange { current : start , limit : end , step , } } _ => print_usage (names , Some (format ! ("Invalid value for {name}: {value}"))) , } }
};
}
