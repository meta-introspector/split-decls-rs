// Generated macro for impl_42 (impl)
macro_rules! Depcrate_tests_packedpairimpl_42 {
() => {
// Module: crate::tests::packedpair
// Provides: {"impl_42"}
// Dependencies: {}
impl Seed { const NEEDLE_LENGTH_LIMIT : usize = { # [cfg (not (miri))] { 33 } # [cfg (miri)] { 5 } } ; const HAYSTACK_LENGTH_LIMIT : usize = { # [cfg (not (miri))] { 65 } # [cfg (miri)] { 8 } } ; # [doc = " Generate a series of prefilter tests from this seed."] fn generate (self) -> impl Iterator < Item = Test > { let len_start = 2 ; (len_start ..= Seed :: NEEDLE_LENGTH_LIMIT) . flat_map (move | needle_len | { let index_start = len_start - 1 ; (index_start .. needle_len) . flat_map (move | index1 | { (index1 .. needle_len) . flat_map (move | index2 | { (needle_len ..= Seed :: HAYSTACK_LENGTH_LIMIT) . flat_map (move | haystack_len | { Test :: new (self , index1 , index2 , haystack_len , needle_len , None ,) . into_iter () . chain ((0 ..= (haystack_len - needle_len)) . flat_map (move | output | { Test :: new (self , index1 , index2 , haystack_len , needle_len , Some (output) ,) } ,) ,) } ,) }) }) }) } }
};
}
