// Generated macro for impl_21 (impl)
macro_rules! Depcrate_sealedimpl_21 {
() => {
// Module: crate::sealed
// Provides: {"impl_21"}
// Dependencies: {}
impl Sealed for super :: Eager { type Pos = () ; type Overhead = U0 ; const NAME : & 'static str = "BlockBuffer<Eager>" ; fn get_pos < N : ArraySize > (buf : & Block < N > , _pos : & Self :: Pos) -> usize { let pos = unsafe { let buf_ptr = buf . as_ptr () . cast :: < u8 > () ; let last_byte_ptr = buf_ptr . add (N :: USIZE - 1) ; ptr :: read (last_byte_ptr) } ; pos as usize } fn set_pos < N : ArraySize > (buf : & mut Block < N > , _pos : & mut Self :: Pos , val : usize) { debug_assert ! (val <= u8 :: MAX as usize) ; unsafe { let buf_ptr = buf . as_mut_ptr () . cast :: < u8 > () ; let last_byte_ptr = buf_ptr . add (N :: USIZE - 1) ; ptr :: write (last_byte_ptr , val as u8) ; } } # [inline (always)] fn invariant (pos : usize , block_size : usize) -> bool { pos < block_size } # [inline (always)] fn split_blocks < N : ArraySize > (data : & [u8]) -> (& [Array < u8 , N >] , & [u8]) { Array :: slice_as_chunks (data) } }
};
}
