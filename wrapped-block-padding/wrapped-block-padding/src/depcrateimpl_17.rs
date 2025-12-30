// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl Padding for NoPadding { # [inline] fn raw_pad (block : & mut [u8] , pos : usize) { if pos > block . len () { panic ! ("`pos` is bigger than block size") ; } } # [inline] fn raw_unpad (block : & [u8]) -> Result < & [u8] , Error > { Ok (block) } # [inline] fn pad_detached < BlockSize : ArraySize > (data : & [u8]) -> PaddedData < '_ , BlockSize > { let (blocks , tail) = Array :: slice_as_chunks (data) ; if tail . is_empty () { PaddedData :: NoPad { blocks } } else { PaddedData :: Error } } # [inline] fn unpad_blocks < BlockSize : ArraySize > (blocks : & [Array < u8 , BlockSize >]) -> Result < & [u8] , Error > { Ok (Array :: slice_as_flattened (blocks)) } }
};
}
