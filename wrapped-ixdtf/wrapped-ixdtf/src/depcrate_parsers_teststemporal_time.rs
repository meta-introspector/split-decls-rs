// Generated macro for temporal_time (function)
macro_rules! Depcrate_parsers_teststemporal_time {
() => {
// Module: crate::parsers::tests
// Provides: {"temporal_time"}
// Dependencies: {}
# [test] fn temporal_time () { let possible_times = ["T12:01:04" , "t12:01:04" , "12:01:04" , "12:01:04[u-ca=iso8601]" , "12:01:04[+04:00][u-ca=iso8601]" , "12:01:04-05:00[America/from_str_York][u-ca=iso8601]" ,] ; for time in possible_times { let result = IxdtfParser :: from_str (time) . parse_time () . unwrap () ; let time = result . time . unwrap () ; assert_eq ! (time . hour , 12) ; assert_eq ! (time . minute , 1) ; assert_eq ! (time . second , 4) ; } }
};
}
