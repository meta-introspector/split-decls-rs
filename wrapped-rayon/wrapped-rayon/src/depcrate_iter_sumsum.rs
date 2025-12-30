// Generated macro for sum (function)
macro_rules! Depcrate_iter_sumsum {
() => {
// Module: crate::iter::sum
// Provides: {"sum"}
// Dependencies: {}
pub (super) fn sum < PI , S > (pi : PI) -> S where PI : ParallelIterator , S : Send + Sum < PI :: Item > + Sum , { pi . drive_unindexed (SumConsumer :: new ()) }
};
}
