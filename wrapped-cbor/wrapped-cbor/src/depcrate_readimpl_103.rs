// Generated macro for impl_103 (impl)
macro_rules! Depcrate_readimpl_103 {
() => {
// Module: crate::read
// Provides: {"impl_103"}
// Dependencies: {}
impl < 'a , 'b > SliceReadFixed < 'a , 'b > { # [doc = " Creates a CBOR input source to read from a slice of bytes, backed by a scratch buffer."] pub fn new (slice : & 'a [u8] , scratch : & 'b mut [u8]) -> SliceReadFixed < 'a , 'b > { SliceReadFixed { slice , scratch , index : 0 , scratch_index : 0 , } } fn end (& self , n : usize) -> Result < usize > { match self . index . checked_add (n) { Some (end) if end <= self . slice . len () => Ok (end) , _ => Err (Error :: syntax (ErrorCode :: EofWhileParsingValue , self . slice . len () as u64 ,)) , } } fn scratch_end (& self , n : usize) -> Result < usize > { match self . scratch_index . checked_add (n) { Some (end) if end <= self . scratch . len () => Ok (end) , _ => Err (Error :: scratch_too_small (self . index as u64)) , } } }
};
}
