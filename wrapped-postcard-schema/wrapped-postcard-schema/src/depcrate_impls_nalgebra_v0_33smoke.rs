// Generated macro for smoke (function)
macro_rules! Depcrate_impls_nalgebra_v0_33smoke {
() => {
// Module: crate::impls::nalgebra_v0_33
// Provides: {"smoke"}
// Dependencies: {}
# [test] fn smoke () { let x = nalgebra_v0_33 :: SMatrix :: < u8 , 3 , 3 > :: new (1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9) ; let y = postcard :: to_stdvec (& x) . unwrap () ; assert_eq ! (& [1 , 4 , 7 , 2 , 5 , 8 , 3 , 6 , 9] , y . as_slice ()) ; }
};
}
