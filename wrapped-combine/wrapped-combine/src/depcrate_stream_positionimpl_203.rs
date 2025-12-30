// Generated macro for impl_203 (impl)
macro_rules! Depcrate_stream_positionimpl_203 {
() => {
// Module: crate::stream::position
// Provides: {"impl_203"}
// Dependencies: {}
impl < Input , X , S > StreamOnce for Stream < Input , X > where Input : StreamOnce , X : Positioner < Input :: Token > , S : StreamError < Input :: Token , Input :: Range > , Input :: Error : ParseError < Input :: Token , Input :: Range , X :: Position , StreamError = S > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position , StreamError = S > , { type Token = Input :: Token ; type Range = Input :: Range ; type Position = X :: Position ; type Error = Input :: Error ; # [inline] fn uncons (& mut self) -> Result < Input :: Token , StreamErrorFor < Self > > { self . input . uncons () . map (| c | { self . positioner . update (& c) ; c }) } fn is_partial (& self) -> bool { self . input . is_partial () } }
};
}
