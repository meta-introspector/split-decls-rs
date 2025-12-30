// Generated macro for tests (module)
macro_rules! Depcrate_extensions_unicodetests {
() => {
// Module: crate::extensions::unicode
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_unicode_extension_fromstr () { let ue : Unicode = "u-foo-hc-h12" . parse () . expect ("Failed to parse Unicode") ; assert_eq ! (ue . to_string () , "u-foo-hc-h12") ; let ue : Result < Unicode , _ > = "u" . parse () ; assert ! (ue . is_err ()) ; } }
};
}
