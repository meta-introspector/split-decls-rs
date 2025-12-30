// Generated macro for CombineRead (trait)
macro_rules! Depcrate_stream_buf_readerCombineRead {
() => {
// Module: crate::stream::buf_reader
// Provides: {"CombineRead"}
// Dependencies: {}
# [cfg (any (feature = "tokio-02" , feature = "tokio-03" , feature = "tokio"))] # [doc (hidden)] pub trait CombineRead < R , T : ? Sized > : CombineBuffer < R > { fn poll_extend_buf (& mut self , cx : & mut Context < '_ > , read : Pin < & mut R > ,) -> Poll < io :: Result < usize > > ; }
};
}
