// Generated macro for test (module)
macro_rules! Depcrate_cryptotest {
() => {
// Module: crate::crypto
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: util :: B32 ; use hex_literal :: hex ; # [test] fn g () { let input = b"hello world" ; let expected1 = hex ! ("3a9159f071e4dd1c8c4f968607c30942e120d8156b8b1e72e0d376e8871cb8b8") ; let expected2 = hex ! ("99072665674f26cc494a4bcf027c58267e8ee2da60e942759de86d2670bba1aa") ; let mut g = G :: default () . absorb (input) ; let mut actual = [0u8 ; 32] ; g . squeeze (& mut actual) ; assert_eq ! (actual , expected1) ; let actual : B32 = g . squeeze_new () ; assert_eq ! (actual , expected2) ; } # [test] fn h () { let input = b"hello world" ; let expected1 = hex ! ("369771bb2cb9d2b04c1d54cca487e372d9f187f73f7ba3f65b95c8ee7798c527") ; let expected2 = hex ! ("f4f3c2d55c2d46a29f2e945d469c3df27853a8735271f5cc2d9e889544357116") ; let mut h = H :: default () . absorb (input) ; let mut actual = [0u8 ; 32] ; h . squeeze (& mut actual) ; assert_eq ! (actual , expected1) ; let actual : B32 = h . squeeze_new () ; assert_eq ! (actual , expected2) ; } }
};
}
