// Generated macro for str (function)
macro_rules! Depcrate_testsstr {
() => {
// Module: crate::tests
// Provides: {"str"}
// Dependencies: {}
# [test] fn str () { defmt :: info ! ("Hello, {=str}" , "world") ; let world = defmt :: intern ! ("world") ; defmt :: info ! ("Hello, {=istr}" , world) ; }
};
}
