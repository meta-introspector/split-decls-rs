// Generated macro for impl_22 (impl)
macro_rules! Depcrate_sealedimpl_22 {
() => {
// Module: crate::sealed
// Provides: {"impl_22"}
// Dependencies: {}
impl Sealed for super :: Lazy { type Pos = u8 ; type Overhead = U1 ; const NAME : & 'static str = "BlockBuffer<Lazy>" ; fn get_pos < N : ArraySize > (_buf_val : & Block < N > , pos : & Self :: Pos) -> usize { * pos as usize } fn set_pos < N : ArraySize > (_ : & mut Block < N > , pos : & mut Self :: Pos , val : usize) { debug_assert ! (val <= u8 :: MAX as usize) ; * pos = val as u8 ; } # [inline (always)] fn invariant (pos : usize , block_size : usize) -> bool { pos <= block_size } # [inline (always)] fn split_blocks < N : ArraySize > (data : & [u8]) -> (& [Array < u8 , N >] , & [u8]) { let (blocks , tail) = Array :: slice_as_chunks (data) ; if data . is_empty () || ! tail . is_empty () { (blocks , tail) } else { let (tail , blocks) = blocks . split_last () . expect ("`blocks` can not be empty") ; (blocks , tail) } } }
};
}
