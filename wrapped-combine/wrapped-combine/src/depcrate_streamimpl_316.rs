// Generated macro for impl_316 (impl)
macro_rules! Depcrate_streamimpl_316 {
() => {
// Module: crate::stream
// Provides: {"impl_316"}
// Dependencies: {}
impl < S > RangeStreamOnce for CompleteStream < S > where S : RangeStreamOnce , { # [inline] fn uncons_range (& mut self , size : usize) -> Result < Self :: Range , StreamErrorFor < Self > > { self . 0 . uncons_range (size) } # [inline] fn uncons_while < F > (& mut self , f : F) -> Result < Self :: Range , StreamErrorFor < Self > > where F : FnMut (Self :: Token) -> bool , { self . 0 . uncons_while (f) } fn uncons_while1 < F > (& mut self , f : F) -> ParseResult < Self :: Range , StreamErrorFor < Self > > where F : FnMut (Self :: Token) -> bool , { self . 0 . uncons_while1 (f) } # [inline] fn distance (& self , end : & Self :: Checkpoint) -> usize { self . 0 . distance (end) } # [inline] fn range (& self) -> Self :: Range { self . 0 . range () } }
};
}
