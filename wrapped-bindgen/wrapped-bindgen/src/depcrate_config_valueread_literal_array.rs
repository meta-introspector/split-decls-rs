// Generated macro for read_literal_array (function)
macro_rules! Depcrate_config_valueread_literal_array {
() => {
// Module: crate::config::value
// Provides: {"read_literal_array"}
// Dependencies: {}
fn read_literal_array (input : & str , len : usize) -> (Vec < & str > , & str) { let mut input = read_token (input , b'{') ; let mut result = vec ! [] ; for _ in 0 .. len { let (literal , rest) = read_literal (input) ; result . push (literal) ; input = rest ; } (result , read_token (input , b'}')) }
};
}
