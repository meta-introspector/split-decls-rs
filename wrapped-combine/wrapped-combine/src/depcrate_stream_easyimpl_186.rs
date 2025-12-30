// Generated macro for impl_186 (impl)
macro_rules! Depcrate_stream_easyimpl_186 {
() => {
// Module: crate::stream::easy
// Provides: {"impl_186"}
// Dependencies: {}
impl < S > RangeStreamOnce for Stream < S > where S : RangeStream , S :: Token : PartialEq , S :: Range : PartialEq , { # [inline] fn uncons_range (& mut self , size : usize) -> Result < Self :: Range , StreamErrorFor < Self > > { self . 0 . uncons_range (size) . map_err (StreamError :: into_other) } # [inline] fn uncons_while < F > (& mut self , f : F) -> Result < Self :: Range , StreamErrorFor < Self > > where F : FnMut (Self :: Token) -> bool , { self . 0 . uncons_while (f) . map_err (StreamError :: into_other) } # [inline] fn uncons_while1 < F > (& mut self , f : F) -> ParseResult < Self :: Range , StreamErrorFor < Self > > where F : FnMut (Self :: Token) -> bool , { self . 0 . uncons_while1 (f) . map_err (StreamError :: into_other) } # [inline] fn distance (& self , end : & Self :: Checkpoint) -> usize { self . 0 . distance (end) } fn range (& self) -> Self :: Range { self . 0 . range () } }
};
}
