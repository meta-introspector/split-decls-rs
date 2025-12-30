// Generated macro for combine_seq (function)
macro_rules! Depcrate_loops_never_loopcombine_seq {
() => {
// Module: crate::loops::never_loop
// Provides: {"combine_seq"}
// Dependencies: {}
# [must_use] fn combine_seq (first : NeverLoopResult , second : impl FnOnce () -> NeverLoopResult) -> NeverLoopResult { match first { NeverLoopResult :: Diverging { .. } | NeverLoopResult :: MayContinueMainLoop => first , NeverLoopResult :: Normal => second () , } }
};
}
