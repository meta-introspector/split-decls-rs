// Generated macro for impl_98 (impl)
macro_rules! Depcrate_readimpl_98 {
() => {
// Module: crate::read
// Provides: {"impl_98"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'a > SliceRead < 'a > { # [doc = " Creates a CBOR input source to read from a slice of bytes."] pub fn new (slice : & 'a [u8]) -> SliceRead < 'a > { SliceRead { slice , scratch : vec ! [] , index : 0 , } } fn end (& self , n : usize) -> Result < usize > { match self . index . checked_add (n) { Some (end) if end <= self . slice . len () => Ok (end) , _ => Err (Error :: syntax (ErrorCode :: EofWhileParsingValue , self . slice . len () as u64 ,)) , } } }
};
}
