// Generated macro for impl_201 (impl)
macro_rules! Depcrate_stream_positionimpl_201 {
() => {
// Module: crate::stream::position
// Provides: {"impl_201"}
// Dependencies: {}
impl < Input > Stream < Input , Input :: Positioner > where Input : StreamOnce + DefaultPositioned , Input :: Positioner : Positioner < Input :: Token > , { # [doc = " Creates a new `Stream<Input, X>` from an input stream and its default positioner."] pub fn new (input : Input) -> Stream < Input , Input :: Positioner > { Stream :: with_positioner (input , Input :: Positioner :: default ()) } }
};
}
