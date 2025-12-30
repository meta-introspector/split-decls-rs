// Generated macro for impl_217 (impl)
macro_rules! Depcrate_stream_positionimpl_217 {
() => {
// Module: crate::stream::position
// Provides: {"impl_217"}
// Dependencies: {}
impl < Input , X , S > RangeStreamOnce for Stream < Input , X > where Input : RangeStreamOnce , X : RangePositioner < Input :: Token , Input :: Range > , S : StreamError < Input :: Token , Input :: Range > , Input :: Error : ParseError < Input :: Token , Input :: Range , X :: Position , StreamError = S > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position , StreamError = S > , Input :: Position : Clone + Ord , { # [inline] fn uncons_range (& mut self , size : usize) -> Result < Input :: Range , StreamErrorFor < Self > > { self . input . uncons_range (size) . map (| range | { self . positioner . update_range (& range) ; range }) } # [inline] fn uncons_while < F > (& mut self , mut predicate : F) -> Result < Input :: Range , StreamErrorFor < Self > > where F : FnMut (Input :: Token) -> bool , { let positioner = & mut self . positioner ; self . input . uncons_while (| t | { if predicate (t . clone ()) { positioner . update (& t) ; true } else { false } }) } # [inline] fn uncons_while1 < F > (& mut self , mut predicate : F ,) -> ParseResult < Self :: Range , StreamErrorFor < Self > > where F : FnMut (Self :: Token) -> bool , { let positioner = & mut self . positioner ; self . input . uncons_while1 (| t | { if predicate (t . clone ()) { positioner . update (& t) ; true } else { false } }) } # [inline] fn distance (& self , end : & Self :: Checkpoint) -> usize { self . input . distance (& end . input) } fn range (& self) -> Self :: Range { self . input . range () } }
};
}
