// Generated macro for impl_257 (impl)
macro_rules! Depcrate_stream_decoderimpl_257 {
() => {
// Module: crate::stream::decoder
// Provides: {"impl_257"}
// Dependencies: {}
impl < E : fmt :: Display , P : fmt :: Display > fmt :: Display for Error < E , P > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: Parse (e) => e . fmt (f) , Error :: Io { position : _ , error } => error . fmt (f) , } } }
};
}
