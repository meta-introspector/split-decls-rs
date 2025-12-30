// Generated macro for tests (module)
macro_rules! Depcrate_ext_impls_impl_zeroizetests {
() => {
// Module: crate::ext_impls::impl_zeroize
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_zeroize () { let mut array = GenericArray :: < u8 , typenum :: U2 > :: default () ; array [0] = 4 ; array [1] = 9 ; array . zeroize () ; assert_eq ! (array [0] , 0) ; assert_eq ! (array [1] , 0) ; } }
};
}
