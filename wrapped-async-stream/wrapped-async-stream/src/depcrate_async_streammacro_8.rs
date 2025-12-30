// Generated macro for macro_8 (macro)
macro_rules! Depcrate_async_streammacro_8 {
() => {
// Module: crate::async_stream
// Provides: {"macro_8"}
// Dependencies: {}
pin_project ! { # [doc (hidden)] # [derive (Debug)] pub struct AsyncStream < T , U > { rx : Receiver < T >, done : bool , # [pin] generator : U , } }
};
}
