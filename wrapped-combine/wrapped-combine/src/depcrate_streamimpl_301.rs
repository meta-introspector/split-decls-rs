// Generated macro for impl_301 (impl)
macro_rules! Depcrate_streamimpl_301 {
() => {
// Module: crate::stream
// Provides: {"impl_301"}
// Dependencies: {}
impl < 'a , T > RangeStreamOnce for & 'a [T] where T : Clone + PartialEq , { # [inline] fn uncons_range (& mut self , size : usize) -> Result < & 'a [T] , StreamErrorFor < Self > > { if size <= self . len () { let (result , remaining) = self . split_at (size) ; * self = remaining ; Ok (result) } else { Err (UnexpectedParse :: Eoi) } } # [inline] fn uncons_while < F > (& mut self , f : F) -> Result < & 'a [T] , StreamErrorFor < Self > > where F : FnMut (Self :: Token) -> bool , { Ok (slice_uncons_while (self , UnconsStart :: Zero , f)) } # [inline] fn uncons_while1 < F > (& mut self , mut f : F) -> ParseResult < Self :: Range , StreamErrorFor < Self > > where F : FnMut (Self :: Token) -> bool , { match self . first () { Some (c) => { if ! f (c . clone ()) { return PeekErr (Tracked :: from (UnexpectedParse :: Unexpected)) ; } } None => { return PeekErr (Tracked :: from (UnexpectedParse :: Eoi)) ; } } CommitOk (slice_uncons_while (self , UnconsStart :: One , f)) } # [inline] fn distance (& self , end : & Self) -> usize { end . len () - self . len () } fn range (& self) -> Self :: Range { self } }
};
}
