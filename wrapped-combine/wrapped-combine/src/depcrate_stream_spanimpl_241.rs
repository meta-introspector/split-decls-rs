// Generated macro for impl_241 (impl)
macro_rules! Depcrate_stream_spanimpl_241 {
() => {
// Module: crate::stream::span
// Provides: {"impl_241"}
// Dependencies: {}
impl < S , E > RangeStreamOnce for Stream < S , E > where S : RangeStream , S :: Token : PartialEq , S :: Range : PartialEq , E : crate :: error :: ParseError < S :: Token , S :: Range , Span < S :: Position > > , S :: Error : ParseErrorInto < S :: Token , S :: Range , S :: Position > , < S :: Error as crate :: error :: ParseError < S :: Token , S :: Range , S :: Position > > :: StreamError : StreamErrorInto < S :: Token , S :: Range > , { # [inline] fn uncons_range (& mut self , size : usize) -> Result < Self :: Range , StreamErrorFor < Self > > { self . 0 . uncons_range (size) . map_err (StreamErrorInto :: into_other_error) } # [inline] fn uncons_while < F > (& mut self , f : F) -> Result < Self :: Range , StreamErrorFor < Self > > where F : FnMut (Self :: Token) -> bool , { self . 0 . uncons_while (f) . map_err (StreamErrorInto :: into_other_error) } # [inline] fn uncons_while1 < F > (& mut self , f : F) -> ParseResult < Self :: Range , StreamErrorFor < Self > > where F : FnMut (Self :: Token) -> bool , { self . 0 . uncons_while1 (f) . map_err (StreamErrorInto :: into_other_error) } # [inline] fn distance (& self , end : & Self :: Checkpoint) -> usize { self . 0 . distance (end) } fn range (& self) -> Self :: Range { self . 0 . range () } }
};
}
