// Generated macro for tests (module)
macro_rules! Depcrate_sfc32tests {
() => {
// Module: crate::sfc32
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn u64_seed () { let reference_rng = Sfc32 { a : 0x09BC85E1 , b : 0x5A96CB07 , c : 0xB53C149C , weyl : 0x10 , } ; let test_rng = Sfc32 :: seed_from_u64 (1) ; assert_eq ! (test_rng , reference_rng) } # [test] fn reference () { let mut rng = Sfc32 :: from_seed ([0 , 0 , 0 , 0 , 2 , 0 , 0 , 0 , 1 , 0 , 0 , 0]) ; # [rustfmt :: skip] let expected : [u32 ; 16] = [0x03B80BB8 , 0xA87DBC7E , 0x1787178C , 0x4C7B7234 , 0xC65DADE2 , 0x2C692349 , 0xF52C2153 , 0xDF098072 , 0x9D49B03C , 0x9562381A , 0xC9B41738 , 0x64B75E54 , 0x36CE9B32 , 0xF106947E , 0x0AFC726B , 0x549BBC87 ,] ; for & e in & expected { assert_eq ! (rng . next_u32 () , e) ; } } }
};
}
