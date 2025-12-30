// Generated macro for test (module)
macro_rules! Depcrate_rngs_reseedingtest {
() => {
// Module: crate::rngs::reseeding
// Provides: {"test"}
// Dependencies: {}
# [cfg (feature = "std_rng")] # [cfg (test)] mod test { use crate :: Rng ; use crate :: rngs :: std :: Core ; use crate :: test :: const_rng ; use super :: ReseedingRng ; # [test] fn test_reseeding () { let zero = const_rng (0) ; let thresh = 1 ; let mut reseeding = ReseedingRng :: < Core , _ > :: new (thresh , zero) . unwrap () ; let mut buf = ([0u32 ; 32] , [0u32 ; 32]) ; reseeding . fill (& mut buf . 0) ; reseeding . fill (& mut buf . 1) ; let seq = buf ; for _ in 0 .. 10 { reseeding . fill (& mut buf . 0) ; reseeding . fill (& mut buf . 1) ; assert_eq ! (buf , seq) ; } } }
};
}
