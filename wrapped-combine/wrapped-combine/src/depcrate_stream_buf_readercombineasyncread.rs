// Generated macro for CombineAsyncRead (trait)
macro_rules! Depcrate_stream_buf_readerCombineAsyncRead {
() => {
// Module: crate::stream::buf_reader
// Provides: {"CombineAsyncRead"}
// Dependencies: {}
# [cfg (feature = "futures-03")] # [doc (hidden)] pub trait CombineAsyncRead < R > : CombineBuffer < R > { fn poll_extend_buf (& mut self , cx : & mut Context < '_ > , read : Pin < & mut R > ,) -> Poll < io :: Result < usize > > ; fn extend_buf < 'a > (& 'a mut self , read : Pin < & 'a mut R >) -> ExtendBuf < 'a , Self , R > where Self : Sized ; }
};
}
