// Generated macro for assert_impl (macro)
macro_rules! Depcrateassert_impl {
() => {
// Module: crate
// Provides: {"assert_impl"}
// Dependencies: {}
# [macro_export] macro_rules ! assert_impl { ($ trait : path : $ ($ ty : ty) ,+) => { { struct Helper < T > (T) ; trait AssertImpl { fn assert () { } } impl < T : $ trait > AssertImpl for Helper < T > { } $ (Helper ::<$ ty >:: assert () ;) + } } ; (!$ trait : path : $ ($ ty : ty) ,+) => { { struct Helper < T > (T) ; trait AssertImpl { fn assert () { } } impl < T : $ trait > AssertImpl for Helper < T > { } trait AssertNotImpl { fn assert () { } } $ (impl AssertNotImpl for Helper <$ ty > { } Helper ::<$ ty >:: assert () ;) + } } ; ($ trait : path : $ ($ ty : ty ,) +) => (assert_impl ! ($ trait : $ ($ ty) ,+)) ; (!$ trait : path : $ ($ ty : ty ,) +) => (assert_impl ! (!$ trait : $ ($ ty) ,+)) ; }
};
}
