// Generated macro for impl_200 (impl)
macro_rules! Depcrate_stream_positionimpl_200 {
() => {
// Module: crate::stream::position
// Provides: {"impl_200"}
// Dependencies: {}
impl < Input , X > Stream < Input , X > where Input : StreamOnce , X : Positioner < Input :: Token > , { # [doc = " Creates a new `Stream<Input, X>` from an input stream and a positioner."] pub fn with_positioner (input : Input , positioner : X) -> Stream < Input , X > { Stream { input , positioner } } }
};
}
