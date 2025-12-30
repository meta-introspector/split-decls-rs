// Generated macro for impl_218 (impl)
macro_rules! Depcrate_stream_positionimpl_218 {
() => {
// Module: crate::stream::position
// Provides: {"impl_218"}
// Dependencies: {}
impl < Input , X , S > ResetStream for Stream < Input , X > where Input : ResetStream , X : Positioner < Input :: Token > , S : StreamError < Input :: Token , Input :: Range > , Input :: Error : ParseError < Input :: Token , Input :: Range , X :: Position , StreamError = S > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position , StreamError = S > , { type Checkpoint = Stream < Input :: Checkpoint , X :: Checkpoint > ; fn checkpoint (& self) -> Self :: Checkpoint { Stream { input : self . input . checkpoint () , positioner : self . positioner . checkpoint () , } } fn reset (& mut self , checkpoint : Self :: Checkpoint) -> Result < () , Self :: Error > { self . input . reset (checkpoint . input) ? ; self . positioner . reset (checkpoint . positioner) ; Ok (()) } }
};
}
