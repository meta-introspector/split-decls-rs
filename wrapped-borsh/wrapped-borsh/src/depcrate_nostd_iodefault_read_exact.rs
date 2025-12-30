// Generated macro for default_read_exact (function)
macro_rules! Depcrate_nostd_iodefault_read_exact {
() => {
// Module: crate::nostd_io
// Provides: {"default_read_exact"}
// Dependencies: {}
fn default_read_exact < R : Read + ? Sized > (this : & mut R , mut buf : & mut [u8]) -> Result < () > { while ! buf . is_empty () { match this . read (buf) { Ok (0) => break , Ok (n) => { let tmp = buf ; buf = & mut tmp [n ..] ; } Err (ref e) if e . kind () == ErrorKind :: Interrupted => { } Err (e) => return Err (e) , } } if ! buf . is_empty () { Err (Error :: new (ErrorKind :: UnexpectedEof , "failed to fill whole buffer" ,)) } else { Ok (()) } }
};
}
