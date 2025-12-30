// Generated macro for get_interface_ident_suffix (function)
macro_rules! Depcrateget_interface_ident_suffix {
() => {
// Module: crate
// Provides: {"get_interface_ident_suffix"}
// Dependencies: {}
fn get_interface_ident_suffix (type_name : & str) -> String { let mut suffix = String :: new () ; for c in type_name . chars () { let c = c . to_ascii_lowercase () ; if suffix . len () >= 20 { break ; } if c . is_ascii_alphanumeric () { suffix . push (c) ; } } suffix }
};
}
