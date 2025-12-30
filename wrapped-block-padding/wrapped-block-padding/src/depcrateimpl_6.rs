// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
impl Padding for ZeroPadding { # [inline] fn raw_pad (block : & mut [u8] , pos : usize) { if pos > block . len () { panic ! ("`pos` is bigger than block size") ; } block [pos ..] . fill (0) ; } # [inline] fn raw_unpad (block : & [u8]) -> Result < & [u8] , Error > { for i in (0 .. block . len ()) . rev () { if block [i] != 0 { return Ok (& block [.. i + 1]) ; } } Ok (& block [.. 0]) } # [inline] fn pad_detached < BlockSize : ArraySize > (data : & [u8]) -> PaddedData < '_ , BlockSize > { let (blocks , tail) = Array :: slice_as_chunks (data) ; if tail . is_empty () { return PaddedData :: NoPad { blocks } ; } let mut tail_block = Array :: default () ; let pos = tail . len () ; tail_block [.. pos] . copy_from_slice (tail) ; Self :: pad (& mut tail_block , pos) ; PaddedData :: Pad { blocks , tail_block } } # [inline] fn unpad_blocks < BlockSize : ArraySize > (blocks : & [Array < u8 , BlockSize >]) -> Result < & [u8] , Error > { let buf = Array :: slice_as_flattened (blocks) ; for i in (0 .. buf . len ()) . rev () { if buf [i] != 0 { return Ok (& buf [.. i + 1]) ; } } Ok (& buf [.. 0]) } }
};
}
