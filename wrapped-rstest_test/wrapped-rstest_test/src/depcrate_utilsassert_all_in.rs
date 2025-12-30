// Generated macro for assert_all_in (macro)
macro_rules! Depcrate_utilsassert_all_in {
() => {
// Module: crate::utils
// Provides: {"assert_all_in"}
// Dependencies: {}
# [macro_export] macro_rules ! assert_all_in { ($ text : expr , $ expected : expr) => (assert_in ! ($ text , $ expected)) ; ($ text : expr , $ expected : expr ,) => (assert_in ! ($ text , $ message)) ; ($ text : expr , $ expected : expr , $ ($ others : expr) ,+) => ({ assert_in ! ($ text , $ expected) ; assert_all_in ! ($ text $ (, $ others) *) ; }) ; }
};
}
