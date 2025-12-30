// Generated macro for combine_seq_many (function)
macro_rules! Depcrate_loops_never_loopcombine_seq_many {
() => {
// Module: crate::loops::never_loop
// Provides: {"combine_seq_many"}
// Dependencies: {}
# [must_use] fn combine_seq_many (iter : impl IntoIterator < Item = NeverLoopResult >) -> NeverLoopResult { for e in iter { if let NeverLoopResult :: Diverging { .. } | NeverLoopResult :: MayContinueMainLoop = e { return e ; } } NeverLoopResult :: Normal }
};
}
