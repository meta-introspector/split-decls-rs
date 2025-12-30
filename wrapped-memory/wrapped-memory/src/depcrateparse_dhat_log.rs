// Generated macro for parse_dhat_log (function)
macro_rules! Depcrateparse_dhat_log {
() => {
// Module: crate
// Provides: {"parse_dhat_log"}
// Dependencies: {}
fn parse_dhat_log (dhat_log : & [String]) -> (u64 , u64 , u64) { assert_eq ! (dhat_log . len () , 4 , "Expected the dhat output to be 4 lines long.") ; (extract_bytes_from_log_line ("dhat: Total:" , & dhat_log [0]) , extract_bytes_from_log_line ("dhat: At t-gmax:" , & dhat_log [1]) , extract_bytes_from_log_line ("dhat: At t-end:" , & dhat_log [2]) ,) }
};
}
