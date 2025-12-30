// Generated macro for tests (module)
macro_rules! Depcrate_rslicetests {
() => {
// Module: crate::rslice
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: rslice :: * ; # [test] fn test_rustls_str_debug () { let s = "abcd" ; let rs : rustls_str = s . try_into () . unwrap () ; assert_eq ! (format ! ("{rs:?}") , r#"rustls_str { data: "abcd", len: 4 }"#) ; } }
};
}
