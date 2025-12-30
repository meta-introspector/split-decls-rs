// Generated macro for is_test (function)
macro_rules! Depcrateis_test {
() => {
// Module: crate
// Provides: {"is_test"}
// Dependencies: {}
# [doc = " Returns true if `file_name` looks like a proper test file name."] pub fn is_test (file_name : & str) -> bool { if ! file_name . ends_with (".rs") { return false ; } let invalid_prefixes = & ["." , "#" , "~"] ; ! invalid_prefixes . iter () . any (| p | file_name . starts_with (p)) }
};
}
