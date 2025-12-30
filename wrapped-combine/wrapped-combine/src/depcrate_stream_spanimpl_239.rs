// Generated macro for impl_239 (impl)
macro_rules! Depcrate_stream_spanimpl_239 {
() => {
// Module: crate::stream::span
// Provides: {"impl_239"}
// Dependencies: {}
impl < S , E > ResetStream for Stream < S , E > where S : ResetStream + Positioned , S :: Token : PartialEq , S :: Range : PartialEq , E : crate :: error :: ParseError < S :: Token , S :: Range , Span < S :: Position > > , S :: Error : ParseErrorInto < S :: Token , S :: Range , S :: Position > , < S :: Error as crate :: error :: ParseError < S :: Token , S :: Range , S :: Position > > :: StreamError : StreamErrorInto < S :: Token , S :: Range > , { type Checkpoint = S :: Checkpoint ; # [inline] fn checkpoint (& self) -> Self :: Checkpoint { self . 0 . checkpoint () } # [inline] fn reset (& mut self , checkpoint : Self :: Checkpoint) -> Result < () , Self :: Error > { self . 0 . reset (checkpoint) . map_err (ParseErrorInto :: into_other_error) } }
};
}
