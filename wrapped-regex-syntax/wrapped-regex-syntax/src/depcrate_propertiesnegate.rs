// Generated macro for negate (function)
macro_rules! Depcrate_propertiesnegate {
() => {
// Module: crate::properties
// Provides: {"negate"}
// Dependencies: {}
# [test] fn negate () { fn prop (ranges : Vec < (char , char) >) -> bool { let expected = class (& ranges) . canonicalize () ; let got = class (& ranges) . negate () . negate () ; expected == got } qc (prop as fn (Vec < (char , char) >) -> bool) ; }
};
}
