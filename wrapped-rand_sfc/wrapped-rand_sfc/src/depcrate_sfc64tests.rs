// Generated macro for tests (module)
macro_rules! Depcrate_sfc64tests {
() => {
// Module: crate::sfc64
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn u64_seed () { let reference_rng = Sfc64 { a : 0xFAFE87BC868CA702 , b : 0x143212184DB2E2BB , c : 0xA4C7E95D7B898700 , weyl : 0x13 , } ; let test_rng = Sfc64 :: seed_from_u64 (1) ; assert_eq ! (test_rng , reference_rng) } # [test] fn reference () { let mut rng = Sfc64 :: from_seed ([1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ,]) ; let expected : [u64 ; 16] = [0xAD6FDC729FEEF3C1 , 0x2A20433D733F77D5 , 0x0310E21369647420 , 0x331A176BC71DCABC , 0x53118F35C2494D94 , 0xA3A99DE7E77E16BF , 0xA7B1B70A3E59A1FF , 0x8E1127B28667EB3C , 0x3FC589DC124CF6E8 , 0x81E0EAAACEB81D81 , 0x79F534652D262DF6 , 0x87F70C8214E186C5 , 0x67AF9C007B825917 , 0x5134AEC9998D8629 , 0x205AA24994068634 , 0x1C762918DBA3E139 ,] ; for & e in & expected { assert_eq ! (rng . next_u64 () , e) ; } } }
};
}
