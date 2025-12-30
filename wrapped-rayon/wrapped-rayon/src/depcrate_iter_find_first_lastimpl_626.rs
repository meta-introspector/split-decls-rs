// Generated macro for impl_626 (impl)
macro_rules! Depcrate_iter_find_first_lastimpl_626 {
() => {
// Module: crate::iter::find_first_last
// Provides: {"impl_626"}
// Dependencies: {}
impl < 'p , P > FindConsumer < 'p , P > { fn new (find_op : & 'p P , match_position : MatchPosition , best_found : & 'p AtomicUsize) -> Self { FindConsumer { find_op , lower_bound : Cell :: new (0) , upper_bound : usize :: MAX , match_position , best_found , } } fn current_index (& self) -> usize { match self . match_position { MatchPosition :: Leftmost => self . lower_bound . get () , MatchPosition :: Rightmost => self . upper_bound , } } }
};
}
