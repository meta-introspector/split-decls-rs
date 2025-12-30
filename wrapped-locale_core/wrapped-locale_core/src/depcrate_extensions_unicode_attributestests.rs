// Generated macro for tests (module)
macro_rules! Depcrate_extensions_unicode_attributestests {
() => {
// Module: crate::extensions::unicode::attributes
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_attributes_fromstr () { let attrs : Attributes = "foo-bar" . parse () . expect ("Failed to parse Attributes") ; assert_eq ! (attrs . to_string () , "bar-foo") ; } }
};
}
