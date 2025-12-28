macro_rules! is_id_start {
    () => {
        # [doc = " True if `c` is valid as a first character of an identifier."] # [doc = " See [Rust language reference](https://doc.rust-lang.org/reference/identifiers.html) for"] # [doc = " a formal definition of valid identifier name."] pub fn is_id_start (c : char) -> bool { c == '_' || unicode_xid :: UnicodeXID :: is_xid_start (c) }
    };
}

is_id_start!();