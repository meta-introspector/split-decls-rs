// Generated macro for assert_components (function)
macro_rules! Depcrate_testsassert_components {
() => {
// Module: crate::tests
// Provides: {"assert_components"}
// Dependencies: {}
fn assert_components (components : & [& str] , path : & RelativePath) { let components = components . iter () . copied () . map (Component :: Normal) . collect :: < Vec < _ > > () ; let result : Vec < _ > = path . components () . collect () ; assert_eq ! (& components [..] , & result [..]) ; }
};
}
