// Generated macro for finite_buffer_fill_buffer (function)
macro_rules! Depcrate_testsfinite_buffer_fill_buffer {
() => {
// Module: crate::tests
// Provides: {"finite_buffer_fill_buffer"}
// Dependencies: {}
# [test] fn finite_buffer_fill_buffer () { let x = [1 , 2 , 3 , 4] ; let mut rb = Unstructured :: new (& x) ; let mut z = [0 ; 2] ; rb . fill_buffer (& mut z) . unwrap () ; assert_eq ! (z , [1 , 2]) ; rb . fill_buffer (& mut z) . unwrap () ; assert_eq ! (z , [3 , 4]) ; rb . fill_buffer (& mut z) . unwrap () ; assert_eq ! (z , [0 , 0]) ; }
};
}
