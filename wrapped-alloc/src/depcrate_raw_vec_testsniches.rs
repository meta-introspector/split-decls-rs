// Generated macro for niches (function)
macro_rules! Depcrate_raw_vec_testsniches {
() => {
// Module: crate::raw_vec::tests
// Provides: {"niches"}
// Dependencies: {}
# [test] fn niches () { let baseline = size_of :: < RawVec < u8 > > () ; assert_eq ! (size_of ::< Option < RawVec < u8 >>> () , baseline) ; assert_eq ! (size_of ::< Option < Option < RawVec < u8 >>>> () , baseline) ; assert_eq ! (size_of ::< Option < Option < Option < RawVec < u8 >>>>> () , baseline) ; }
};
}
