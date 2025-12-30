// Generated macro for CStringLit (struct)
macro_rules! Depcrate_cstrCStringLit {
() => {
// Module: crate::cstr
// Provides: {"CStringLit"}
// Dependencies: {}
# [doc = " A C string or raw C string literal, e.g. `c\"hello\"` or `cr#\"abc\"def\"#`."] # [doc = ""] # [doc = " See [the reference][ref] for more information."] # [doc = ""] # [doc = " [ref]: https://doc.rust-lang.org/reference/tokens.html#c-string-and-raw-c-string-literals"] # [derive (Debug , Clone , PartialEq , Eq)] pub struct CStringLit < B : Buffer > { # [doc = " The raw input."] raw : B , # [doc = " The string value (with all escaped unescaped) as CString. This is not an"] # [doc = " `Option` as we always have to add the trailing zero byte."] value : CString , # [doc = " The number of hash signs in case of a raw string literal, or `None` if"] # [doc = " it's not a raw string literal."] num_hashes : Option < u8 > , # [doc = " Start index of the suffix or `raw.len()` if there is no suffix."] start_suffix : usize , }
};
}
