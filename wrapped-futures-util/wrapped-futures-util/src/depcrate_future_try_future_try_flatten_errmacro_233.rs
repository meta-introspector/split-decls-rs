// Generated macro for macro_233 (macro)
macro_rules! Depcrate_future_try_future_try_flatten_errmacro_233 {
() => {
// Module: crate::future::try_future::try_flatten_err
// Provides: {"macro_233"}
// Dependencies: {}
pin_project ! { # [project = TryFlattenErrProj] # [derive (Debug)] pub enum TryFlattenErr < Fut1 , Fut2 > { First { # [pin] f : Fut1 } , Second { # [pin] f : Fut2 } , Empty , } }
};
}
