// Generated macro for macro_14 (macro)
macro_rules! Depcrate_barriermacro_14 {
() => {
// Module: crate::barrier
// Provides: {"macro_14"}
// Dependencies: {}
easy_wrapper ! { # [doc = " The future returned by [`Barrier::wait()`]."] pub struct BarrierWait <'a > (BarrierWaitInner <'a > => BarrierWaitResult) ; # [cfg (all (feature = "std" , not (target_family = "wasm")))] pub (crate) wait () ; }
};
}
