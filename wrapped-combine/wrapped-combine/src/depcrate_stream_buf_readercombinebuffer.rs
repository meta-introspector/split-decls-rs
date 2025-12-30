// Generated macro for CombineBuffer (trait)
macro_rules! Depcrate_stream_buf_readerCombineBuffer {
() => {
// Module: crate::stream::buf_reader
// Provides: {"CombineBuffer"}
// Dependencies: {}
# [doc (hidden)] pub trait CombineBuffer < R > : sealed :: Sealed { fn buffer < 'a > (& 'a self , read : & 'a R) -> & 'a [u8] ; fn advance (& mut self , read : & mut R , len : usize) ; # [cfg (feature = "pin-project-lite")] fn advance_pin (& mut self , read : Pin < & mut R > , len : usize) ; }
};
}
