// Generated macro for hchacha20_tests (module)
macro_rules! Depcrate_xchachahchacha20_tests {
() => {
// Module: crate::xchacha
// Provides: {"hchacha20_tests"}
// Dependencies: {}
# [cfg (test)] mod hchacha20_tests { use super :: * ; use hex_literal :: hex ; # [doc = " Test vectors from:"] # [doc = " https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-xchacha#section-2.2.1"] # [test] fn test_vector () { const KEY : [u8 ; 32] = hex ! ("000102030405060708090a0b0c0d0e0f" "101112131415161718191a1b1c1d1e1f") ; const INPUT : [u8 ; 16] = hex ! ("000000090000004a0000000031415927") ; const OUTPUT : [u8 ; 32] = hex ! ("82413b4227b27bfed30e42508a877d73" "a0f9e4d58a74a853c12ec41326d3ecdc") ; let actual = hchacha :: < R20 > (& KEY . into () , & INPUT . into ()) ; assert_eq ! (actual . as_slice () , & OUTPUT) ; } }
};
}
