// Generated macro for attrs (function)
macro_rules! Depcrate_testattrs {
() => {
// Module: crate::test
// Provides: {"attrs"}
// Dependencies: {}
pub (crate) fn attrs (s : impl AsRef < str >) -> Vec < syn :: Attribute > { parse_str :: < ItemFn > (& format ! (r#"{}
           fn _no_name_() {{}}
        "# , s . as_ref ())) . unwrap () . attrs }
};
}
