// Generated macro for SEEDS (const)
macro_rules! Depcrate_tests_packedpairSEEDS {
() => {
// Module: crate::tests::packedpair
// Provides: {"SEEDS"}
// Dependencies: {}
# [doc = " A set of \"packed pair\" test seeds. Each seed serves as the base for the"] # [doc = " generation of many other tests. In essence, the seed captures the pair of"] # [doc = " bytes we used for a predicate and first byte among our needle. The tests"] # [doc = " generated from each seed essentially vary the length of the needle and"] # [doc = " haystack, while using the rare/first byte configuration from the seed."] # [doc = ""] # [doc = " The purpose of this is to test many different needle/haystack lengths."] # [doc = " In particular, some of the vector optimizations might only have bugs"] # [doc = " in haystacks of a certain size."] const SEEDS : & [Seed] = & [Seed { first : b'x' , index1 : b'y' , index2 : b'z' } , Seed { first : b'x' , index1 : b'x' , index2 : b'z' } , Seed { first : b'x' , index1 : b'y' , index2 : b'x' } , Seed { first : b'x' , index1 : b'x' , index2 : b'x' } , Seed { first : b'x' , index1 : b'y' , index2 : b'y' } ,] ;
};
}
