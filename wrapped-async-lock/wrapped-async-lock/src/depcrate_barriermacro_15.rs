// Generated macro for macro_15 (macro)
macro_rules! Depcrate_barriermacro_15 {
() => {
// Module: crate::barrier
// Provides: {"macro_15"}
// Dependencies: {}
pin_project_lite :: pin_project ! { # [doc = " The future returned by [`Barrier::wait()`]."] struct BarrierWaitInner <'a > { barrier : &'a Barrier , # [pin] lock : Option < Lock <'a , State >>, evl : Option < EventListener >, state : WaitState , } }
};
}
