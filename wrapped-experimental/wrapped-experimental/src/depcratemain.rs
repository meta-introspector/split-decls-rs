// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let names = RegionDisplayNames :: try_new (locale ! ("fr") . into () , Default :: default () ,) . expect ("locale 'fr' should be present in compiled data") ; let name = names . of (region ! ("DE")) . unwrap () ; assert_eq ! (name , "Allemagne") ; println ! ("{name}") ; }
};
}
