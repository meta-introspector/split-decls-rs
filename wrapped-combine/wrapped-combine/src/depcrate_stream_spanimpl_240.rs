// Generated macro for impl_240 (impl)
macro_rules! Depcrate_stream_spanimpl_240 {
() => {
// Module: crate::stream::span
// Provides: {"impl_240"}
// Dependencies: {}
impl < S , E > StreamOnce for Stream < S , E > where S : StreamOnce + Positioned , S :: Token : PartialEq , S :: Range : PartialEq , E : crate :: error :: ParseError < S :: Token , S :: Range , Span < S :: Position > > , S :: Error : ParseErrorInto < S :: Token , S :: Range , S :: Position > , < S :: Error as crate :: error :: ParseError < S :: Token , S :: Range , S :: Position > > :: StreamError : StreamErrorInto < S :: Token , S :: Range > , { type Token = S :: Token ; type Range = S :: Range ; type Position = Span < S :: Position > ; type Error = E ; # [inline] fn uncons (& mut self) -> Result < Self :: Token , StreamErrorFor < Self > > { self . 0 . uncons () . map_err (StreamErrorInto :: into_other_error) } # [inline] fn is_partial (& self) -> bool { self . 0 . is_partial () } }
};
}
