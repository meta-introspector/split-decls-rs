// Generated macro for test_deeply_nested_mut (function)
macro_rules! Depcratetest_deeply_nested_mut {
() => {
// Module: crate
// Provides: {"test_deeply_nested_mut"}
// Dependencies: {}
# [test] fn test_deeply_nested_mut () { let mut s = Baz { bar : Bar { foo : Foo { a : 1 , b : 2 } , a : 3 , } , a : 7 , } ; let mut sr = s . into_partial_ref_mut () ; split_borrow ! (foo_a = & (mut PartBar | PartFoo | PartA) sr) ; std :: mem :: swap (foo_a . part_mut (PartBar | PartFoo | PartA) , sr . part_mut (PartA) ,) ; assert_eq ! (s . bar . foo . a , 7) ; assert_eq ! (s . a , 1) ; }
};
}
