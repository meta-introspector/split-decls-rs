// Generated macro for impl_36 (impl)
macro_rules! Depcrate_streamimpl_36 {
() => {
// Module: crate::stream
// Provides: {"impl_36"}
// Dependencies: {}
impl fmt :: Display for Error { # [inline] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Error :: Data => "lzma data error" , Error :: Options => "invalid options" , Error :: Format => "stream/file format not recognized" , Error :: MemLimit => "memory limit reached" , Error :: Mem => "can't allocate memory" , Error :: Program => "liblzma internal error" , Error :: NoCheck => "no integrity check was available" , Error :: UnsupportedCheck => "liblzma not built with check support" , } . fmt (f) } }
};
}
