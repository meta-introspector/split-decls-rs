// Generated macro for impl_12 (impl)
macro_rules! Depcrate_stdimpl_12 {
() => {
// Module: crate::std
// Provides: {"impl_12"}
// Dependencies: {}
# [deny (clippy :: missing_trait_methods , reason = "Methods should be forwarded to the underlying type")] impl < T : std :: io :: Read + ? Sized > embedded_io :: Read for FromStd < T > { fn read (& mut self , buf : & mut [u8]) -> Result < usize , Self :: Error > { self . inner . read (buf) } fn read_exact (& mut self , buf : & mut [u8] ,) -> Result < () , embedded_io :: ReadExactError < Self :: Error > > { match self . inner . read_exact (buf) { Ok (()) => Ok (()) , Err (error) if error . kind () == std :: io :: ErrorKind :: UnexpectedEof => { Err (embedded_io :: ReadExactError :: UnexpectedEof) } Err (error) => Err (error . into ()) , } } }
};
}
