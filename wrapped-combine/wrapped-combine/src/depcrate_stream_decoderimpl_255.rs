// Generated macro for impl_255 (impl)
macro_rules! Depcrate_stream_decoderimpl_255 {
() => {
// Module: crate::stream::decoder
// Provides: {"impl_255"}
// Dependencies: {}
impl < 'a , P > From < Error < crate :: easy :: Errors < u8 , & 'a [u8] , P > , P > > for crate :: easy :: Errors < u8 , & 'a [u8] , P > where P : Ord + Clone , { fn from (e : Error < crate :: easy :: Errors < u8 , & 'a [u8] , P > , P >) -> Self { match e { Error :: Parse (e) => e , Error :: Io { position , error } => { crate :: easy :: Errors :: from_error (position , crate :: easy :: Error :: Other (error . into ())) } } } }
};
}
