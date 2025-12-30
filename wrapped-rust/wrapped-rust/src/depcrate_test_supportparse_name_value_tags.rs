// Generated macro for parse_name_value_tags (function)
macro_rules! Depcrate_test_supportparse_name_value_tags {
() => {
// Module: crate::test_support
// Provides: {"parse_name_value_tags"}
// Dependencies: {}
fn parse_name_value_tags (rdns : & Name < '_ >) -> Vec < u8 > { let mut tags = vec ! [] ; for rdn in rdns . unwrap_read () . clone () { let mut attributes = rdn . collect :: < Vec < _ > > () ; assert_eq ! (attributes . len () , 1) ; tags . push (attributes . pop () . unwrap () . value . tag () . as_u8 () . unwrap ()) ; } tags }
};
}
