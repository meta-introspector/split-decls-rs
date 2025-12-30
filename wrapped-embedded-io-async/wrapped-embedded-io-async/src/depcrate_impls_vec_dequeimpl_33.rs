// Generated macro for impl_33 (impl)
macro_rules! Depcrate_impls_vec_dequeimpl_33 {
() => {
// Module: crate::impls::vec_deque
// Provides: {"impl_33"}
// Dependencies: {}
# [doc = " Read is implemented for `VecDeque<u8>` by consuming bytes from the front of the `VecDeque`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "std" , feature = "alloc"))))] impl Read for VecDeque < u8 > { # [doc = " Fill `buf` with the contents of the \"front\" slice as returned by"] # [doc = " [`as_slices`][`VecDeque::as_slices`]. If the contained byte slices of the `VecDeque` are"] # [doc = " discontiguous, multiple calls to `read` will be needed to read the entire content."] # [inline] async fn read (& mut self , buf : & mut [u8]) -> Result < usize , Self :: Error > { let (ref mut front , _) = self . as_slices () ; let n = Read :: read (front , buf) . await ? ; self . drain (.. n) ; Ok (n) } # [inline] async fn read_exact (& mut self , buf : & mut [u8]) -> Result < () , ReadExactError < Self :: Error > > { let (front , back) = self . as_slices () ; match buf . split_at_mut_checked (front . len ()) { None => buf . copy_from_slice (& front [.. buf . len ()]) , Some ((buf_front , buf_back)) => match back . split_at_checked (buf_back . len ()) { Some ((back , _)) => { buf_front . copy_from_slice (front) ; buf_back . copy_from_slice (back) ; } None => { self . clear () ; return Err (ReadExactError :: UnexpectedEof) ; } } , } self . drain (.. buf . len ()) ; Ok (()) } }
};
}
