// Generated macro for macro_491 (macro)
macro_rules! Depcrate_rt_tokiomacro_491 {
() => {
// Module: crate::rt::tokio
// Provides: {"macro_491"}
// Dependencies: {}
pin_project ! { # [doc = " A wrapper that implements Tokio's IO traits for an inner type that"] # [doc = " implements hyper's IO traits, or vice versa (implements hyper's IO"] # [doc = " traits for a type that implements Tokio's IO traits)."] # [derive (Debug)] pub struct TokioIo < T > { # [pin] inner : T , } }
};
}
