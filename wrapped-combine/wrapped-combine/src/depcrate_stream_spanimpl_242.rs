// Generated macro for impl_242 (impl)
macro_rules! Depcrate_stream_spanimpl_242 {
() => {
// Module: crate::stream::span
// Provides: {"impl_242"}
// Dependencies: {}
impl < S , E > Positioned for Stream < S , E > where S : StreamOnce + Positioned , S :: Token : PartialEq , S :: Range : PartialEq , E : crate :: error :: ParseError < S :: Token , S :: Range , Span < S :: Position > > , S :: Error : ParseErrorInto < S :: Token , S :: Range , S :: Position > , < S :: Error as crate :: error :: ParseError < S :: Token , S :: Range , S :: Position > > :: StreamError : StreamErrorInto < S :: Token , S :: Range > , { fn position (& self) -> Span < S :: Position > { Span :: from (self . 0 . position ()) } }
};
}
