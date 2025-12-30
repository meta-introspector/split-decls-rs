// Generated macro for impl_291 (impl)
macro_rules! Depcrate_streamimpl_291 {
() => {
// Module: crate::stream
// Provides: {"impl_291"}
// Dependencies: {}
impl < 'a , I > RangeStreamOnce for & 'a mut I where I : RangeStreamOnce + ? Sized , { # [inline] fn uncons_while < F > (& mut self , f : F) -> Result < Self :: Range , StreamErrorFor < Self > > where F : FnMut (Self :: Token) -> bool , { (* * self) . uncons_while (f) } # [inline] fn uncons_while1 < F > (& mut self , f : F) -> ParseResult < Self :: Range , StreamErrorFor < Self > > where F : FnMut (Self :: Token) -> bool , { (* * self) . uncons_while1 (f) } # [inline] fn uncons_range (& mut self , size : usize) -> Result < Self :: Range , StreamErrorFor < Self > > { (* * self) . uncons_range (size) } # [inline] fn distance (& self , end : & Self :: Checkpoint) -> usize { (* * self) . distance (end) } fn range (& self) -> Self :: Range { (* * self) . range () } }
};
}
