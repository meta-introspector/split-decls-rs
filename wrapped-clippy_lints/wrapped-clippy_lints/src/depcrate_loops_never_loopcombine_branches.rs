// Generated macro for combine_branches (function)
macro_rules! Depcrate_loops_never_loopcombine_branches {
() => {
// Module: crate::loops::never_loop
// Provides: {"combine_branches"}
// Dependencies: {}
# [must_use] fn combine_branches (b1 : NeverLoopResult , b2 : NeverLoopResult) -> NeverLoopResult { match (b1 , b2) { (NeverLoopResult :: MayContinueMainLoop , _) | (_ , NeverLoopResult :: MayContinueMainLoop) => { NeverLoopResult :: MayContinueMainLoop } , (NeverLoopResult :: Normal , _) | (_ , NeverLoopResult :: Normal) => NeverLoopResult :: Normal , (NeverLoopResult :: Diverging { break_spans : mut break_spans1 , never_spans : mut never_spans1 , } , NeverLoopResult :: Diverging { break_spans : mut break_spans2 , never_spans : mut never_spans2 , } ,) => { break_spans1 . append (& mut break_spans2) ; never_spans1 . append (& mut never_spans2) ; NeverLoopResult :: Diverging { break_spans : break_spans1 , never_spans : never_spans1 , } } , } }
};
}
