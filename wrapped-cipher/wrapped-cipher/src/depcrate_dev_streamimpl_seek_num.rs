// Generated macro for impl_seek_num (macro)
macro_rules! Depcrate_dev_streamimpl_seek_num {
() => {
// Module: crate::dev::stream
// Provides: {"impl_seek_num"}
// Dependencies: {}
macro_rules ! impl_seek_num { { $ ($ t : ty) * } => { $ (impl SeekNum for $ t { fn from_block_byte < T : StreamCipherCounter > (block : T , byte : u8 , block_size : u8) -> Result < Self , OverflowError > { debug_assert ! (byte != 0) ; let rem = block_size . checked_sub (byte) . ok_or (OverflowError) ?; let block : Self = block . try_into () . map_err (| _ | OverflowError) ?; block . checked_mul (block_size . into ()) . and_then (| v | v . checked_sub (rem . into ())) . ok_or (OverflowError) } fn into_block_byte < T : StreamCipherCounter > (self , block_size : u8) -> Result < (T , u8) , OverflowError > { let bs = Self :: from (block_size) ; let byte = u8 :: try_from (self % bs) . expect ("bs fits into u8") ; let block = T :: try_from (self / bs) . map_err (| _ | OverflowError) ?; Ok ((block , byte)) } }) * } ; }
};
}
