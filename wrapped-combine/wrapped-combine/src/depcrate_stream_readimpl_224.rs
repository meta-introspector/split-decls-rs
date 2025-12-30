// Generated macro for impl_224 (impl)
macro_rules! Depcrate_stream_readimpl_224 {
() => {
// Module: crate::stream::read
// Provides: {"impl_224"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: Unexpected => write ! (f , "unexpected parse") , Error :: EndOfInput => write ! (f , "unexpected end of input") , Error :: Io (err) => write ! (f , "{}" , err) , } } }
};
}
