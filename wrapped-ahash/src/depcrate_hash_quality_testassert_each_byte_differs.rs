// Generated macro for assert_each_byte_differs (function)
macro_rules! Depcrate_hash_quality_testassert_each_byte_differs {
() => {
// Module: crate::hash_quality_test
// Provides: {"assert_each_byte_differs"}
// Dependencies: {}
fn assert_each_byte_differs (num : u64 , base : u64 , alternatives : Vec < u64 >) { let mut changed_bits = 0_u64 ; for alternative in alternatives { changed_bits |= base ^ alternative } assert_eq ! (core :: u64 :: MAX , changed_bits , "Bits changed: {:x} on num: {:?}. base {:x}" , changed_bits , num , base) ; }
};
}
