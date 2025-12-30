// Generated macro for impl_108 (impl)
macro_rules! Depcrate_readimpl_108 {
() => {
// Module: crate::read
// Provides: {"impl_108"}
// Dependencies: {}
impl < 'a > MutSliceRead < 'a > { # [doc = " Creates a CBOR input source to read from a slice of bytes."] pub fn new (slice : & 'a mut [u8]) -> MutSliceRead < 'a > { MutSliceRead { slice , index : 0 , before : 0 , buffer_end : 0 , } } fn end (& self , n : usize) -> Result < usize > { match self . index . checked_add (n) { Some (end) if end <= self . slice . len () => Ok (end) , _ => Err (Error :: syntax (ErrorCode :: EofWhileParsingValue , self . slice . len () as u64 ,)) , } } }
};
}
