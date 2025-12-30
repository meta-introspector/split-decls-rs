// Generated macro for pick_weighted (function)
macro_rules! Depcrate_strategy_unionspick_weighted {
() => {
// Module: crate::strategy::unions
// Provides: {"pick_weighted"}
// Dependencies: {}
fn pick_weighted < I : Iterator < Item = u32 > > (runner : & mut TestRunner , weights1 : I , weights2 : I ,) -> usize { let sum = weights1 . map (u64 :: from) . sum () ; let weighted_pick = sample_uniform (runner , 0 , sum) ; weights2 . scan (0u64 , | state , w | { * state += u64 :: from (w) ; Some (* state) }) . filter (| & v | v <= weighted_pick) . count () }
};
}
