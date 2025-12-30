// Generated macro for macro_10620 (macro)
macro_rules! Depcrate_typesmacro_10620 {
() => {
// Module: crate::types
// Provides: {"macro_10620"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects needlessly owned `Cow` types."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The borrowed types are usually more flexible, in that e.g. a"] # [doc = " `Cow<'_, str>` can accept both `&str` and `String` while"] # [doc = " `Cow<'_, String>` can only accept `&String` and `String`. In"] # [doc = " particular, `&str` is more general, because it allows for string"] # [doc = " literals while `&String` can only be borrowed from a heap-owned"] # [doc = " `String`)."] # [doc = ""] # [doc = " ### Known Problems"] # [doc = " The lint does not check for usage of the type. There may be external"] # [doc = " interfaces that require the use of an owned type."] # [doc = ""] # [doc = " At least the `CString` type also has a different API than `CStr`: The"] # [doc = " former has an `as_bytes` method which the latter calls `to_bytes`."] # [doc = " There is no guarantee that other types won't gain additional methods"] # [doc = " leading to a similar mismatch."] # [doc = ""] # [doc = " In addition, the lint only checks for the known problematic types"] # [doc = " `String`, `Vec<_>`, `CString`, `OsString` and `PathBuf`. Custom types"] # [doc = " that implement `ToOwned` will not be detected."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let wrogn: std::borrow::Cow<'_, Vec<u8>>;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let right: std::borrow::Cow<'_, [u8]>;"] # [doc = " ```"] # [clippy :: version = "1.87.0"] pub OWNED_COW , style , "needlessly owned Cow type" }
};
}
