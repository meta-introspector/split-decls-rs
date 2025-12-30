// Generated macro for macro_220 (macro)
macro_rules! Depcrate_future_try_future_try_flattenmacro_220 {
() => {
// Module: crate::future::try_future::try_flatten
// Provides: {"macro_220"}
// Dependencies: {}
pin_project ! { # [project = TryFlattenProj] # [derive (Debug)] pub enum TryFlatten < Fut1 , Fut2 > { First { # [pin] f : Fut1 } , Second { # [pin] f : Fut2 } , Empty , } }
};
}
