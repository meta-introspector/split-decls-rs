// Generated macro for DefaultState (struct)
macro_rules! Depcrate_builderDefaultState {
() => {
// Module: crate::builder
// Provides: {"DefaultState"}
// Dependencies: {}
# [doc = " Represents data specific to builder in default, synchronous state, without support for async."] # [derive (Debug , Default , Clone)] pub struct DefaultState { sources : Vec < Box < dyn Source + Send + Sync > > , }
};
}
