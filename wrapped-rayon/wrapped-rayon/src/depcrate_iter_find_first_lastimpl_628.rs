// Generated macro for impl_628 (impl)
macro_rules! Depcrate_iter_find_first_lastimpl_628 {
() => {
// Module: crate::iter::find_first_last
// Provides: {"impl_628"}
// Dependencies: {}
impl < 'p , T , P > UnindexedConsumer < T > for FindConsumer < 'p , P > where T : Send , P : Fn (& T) -> bool + Sync , { fn split_off_left (& self) -> Self { let old_lower_bound = self . lower_bound . get () ; let median = old_lower_bound + ((self . upper_bound - old_lower_bound) / 2) ; self . lower_bound . set (median) ; FindConsumer { find_op : self . find_op , lower_bound : Cell :: new (old_lower_bound) , upper_bound : median , match_position : self . match_position , best_found : self . best_found , } } fn to_reducer (& self) -> Self :: Reducer { FindReducer { match_position : self . match_position , } } }
};
}
