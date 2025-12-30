// Generated macro for macro_85 (macro)
macro_rules! Depcrate_future_future_flattenmacro_85 {
() => {
// Module: crate::future::future::flatten
// Provides: {"macro_85"}
// Dependencies: {}
pin_project ! { # [project = FlattenProj] # [derive (Debug)] pub enum Flatten < Fut1 , Fut2 > { First { # [pin] f : Fut1 } , Second { # [pin] f : Fut2 } , Empty , } }
};
}
