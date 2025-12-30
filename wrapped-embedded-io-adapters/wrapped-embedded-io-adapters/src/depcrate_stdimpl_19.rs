// Generated macro for impl_19 (impl)
macro_rules! Depcrate_stdimpl_19 {
() => {
// Module: crate::std
// Provides: {"impl_19"}
// Dependencies: {}
impl < T : embedded_io :: Read + ? Sized > std :: io :: Read for ToStd < T > { fn read (& mut self , buf : & mut [u8]) -> Result < usize , std :: io :: Error > { self . inner . read (buf) . map_err (to_std_error) } fn read_exact (& mut self , buf : & mut [u8]) -> std :: io :: Result < () > { match self . inner . read_exact (buf) { Ok (()) => Ok (()) , Err (e @ embedded_io :: ReadExactError :: UnexpectedEof) => Err (std :: io :: Error :: new (std :: io :: ErrorKind :: UnexpectedEof , format ! ("{e:?}") ,)) , Err (embedded_io :: ReadExactError :: Other (e)) => Err (to_std_error (e)) , } } }
};
}
