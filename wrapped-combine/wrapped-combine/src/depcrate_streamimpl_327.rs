// Generated macro for impl_327 (impl)
macro_rules! Depcrate_streamimpl_327 {
() => {
// Module: crate::stream
// Provides: {"impl_327"}
// Dependencies: {}
impl < 'a , T > RangeStreamOnce for SliceStream < 'a , T > where T : PartialEq + 'a , { # [inline] fn uncons_range (& mut self , size : usize) -> Result < & 'a [T] , StreamErrorFor < Self > > { if size <= self . 0 . len () { let (range , rest) = self . 0 . split_at (size) ; self . 0 = rest ; Ok (range) } else { Err (UnexpectedParse :: Eoi) } } # [inline] fn uncons_while < F > (& mut self , f : F) -> Result < & 'a [T] , StreamErrorFor < Self > > where F : FnMut (Self :: Token) -> bool , { Ok (slice_uncons_while_ref (& mut self . 0 , UnconsStart :: Zero , f)) } # [inline] fn uncons_while1 < F > (& mut self , mut f : F) -> ParseResult < Self :: Range , StreamErrorFor < Self > > where F : FnMut (Self :: Token) -> bool , { match self . 0 . first () { Some (c) => { if ! f (c) { return PeekErr (Tracked :: from (UnexpectedParse :: Unexpected)) ; } } None => return PeekErr (Tracked :: from (UnexpectedParse :: Eoi)) , } CommitOk (slice_uncons_while_ref (& mut self . 0 , UnconsStart :: One , f)) } # [inline] fn distance (& self , end : & Self) -> usize { end . 0 . len () - self . 0 . len () } fn range (& self) -> Self :: Range { self . 0 } }
};
}
