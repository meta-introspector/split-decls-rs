// Generated macro for impl_249 (impl)
macro_rules! Depcrate_stream_stateimpl_249 {
() => {
// Module: crate::stream::state
// Provides: {"impl_249"}
// Dependencies: {}
impl < S , U > RangeStreamOnce for Stream < S , U > where S : RangeStreamOnce , { # [inline] fn uncons_range (& mut self , size : usize) -> Result < Self :: Range , StreamErrorFor < Self > > { self . stream . uncons_range (size) } # [inline] fn uncons_while < F > (& mut self , f : F) -> Result < Self :: Range , StreamErrorFor < Self > > where F : FnMut (Self :: Token) -> bool , { self . stream . uncons_while (f) } fn uncons_while1 < F > (& mut self , f : F) -> ParseResult < Self :: Range , StreamErrorFor < Self > > where F : FnMut (Self :: Token) -> bool , { self . stream . uncons_while1 (f) } # [inline] fn distance (& self , end : & Self :: Checkpoint) -> usize { self . stream . distance (end) } # [inline] fn range (& self) -> Self :: Range { self . stream . range () } }
};
}
