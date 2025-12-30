// Generated macro for absorb_break (function)
macro_rules! Depcrate_loops_never_loopabsorb_break {
() => {
// Module: crate::loops::never_loop
// Provides: {"absorb_break"}
// Dependencies: {}
# [must_use] fn absorb_break (arg : & NeverLoopResult) -> NeverLoopResult { match arg { NeverLoopResult :: Diverging { .. } | NeverLoopResult :: Normal => NeverLoopResult :: Normal , NeverLoopResult :: MayContinueMainLoop => NeverLoopResult :: MayContinueMainLoop , } }
};
}
