// Generated macro for is_id_continue (function)
macro_rules! Depcrateis_id_continue {
() => {
// Module: crate
// Provides: {"is_id_continue"}
// Dependencies: {}
# [doc = " True if `c` is valid as a non-first character of an identifier."] # [doc = " See [Rust language reference](https://doc.rust-lang.org/reference/identifiers.html) for"] # [doc = " a formal definition of valid identifier name."] pub fn is_id_continue (c : char) -> bool { unicode_xid :: UnicodeXID :: is_xid_continue (c) }
};
}
