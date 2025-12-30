// Generated macro for impl_202 (impl)
macro_rules! Depcrate_stream_positionimpl_202 {
() => {
// Module: crate::stream::position
// Provides: {"impl_202"}
// Dependencies: {}
impl < Input , X , S > Positioned for Stream < Input , X > where Input : StreamOnce , X : Positioner < Input :: Token > , S : StreamError < Input :: Token , Input :: Range > , Input :: Error : ParseError < Input :: Token , Input :: Range , X :: Position , StreamError = S > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position , StreamError = S > , { # [inline] fn position (& self) -> Self :: Position { self . positioner . position () } }
};
}
