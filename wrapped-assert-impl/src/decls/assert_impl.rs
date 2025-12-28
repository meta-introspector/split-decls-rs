macro_rules! assert_impl {
    () => {
        # [macro_export] macro_rules ! assert_impl { ($ trait : path : $ ($ ty : ty) ,+) => { { struct Helper < T > (T) ; trait AssertImpl { fn assert () { } } impl < T : $ trait > AssertImpl for Helper < T > { } $ (Helper ::<$ ty >:: assert () ;) + } } ; (!$ trait : path : $ ($ ty : ty) ,+) => { { struct Helper < T > (T) ; trait AssertImpl { fn assert () { } } impl < T : $ trait > AssertImpl for Helper < T > { } trait AssertNotImpl { fn assert () { } } $ (impl AssertNotImpl for Helper <$ ty > { } Helper ::<$ ty >:: assert () ;) + } } ; ($ trait : path : $ ($ ty : ty ,) +) => (assert_impl ! ($ trait : $ ($ ty) ,+)) ; (!$ trait : path : $ ($ ty : ty ,) +) => (assert_impl ! (!$ trait : $ ($ ty) ,+)) ; }
    };
}

assert_impl!();