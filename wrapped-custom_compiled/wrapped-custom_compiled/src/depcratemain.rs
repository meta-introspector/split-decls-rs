// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let rules = PluralRules :: try_new_cardinal (locale ! ("ru") . into ()) . expect ("locale 'ru' should be present in the compiled data") ; let result = rules . category_for (3) ; assert_eq ! (result , PluralCategory :: Few) ; println ! ("{result:?}") ; }
};
}
