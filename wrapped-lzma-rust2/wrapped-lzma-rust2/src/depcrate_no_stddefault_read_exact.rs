// Generated macro for default_read_exact (function)
macro_rules! Depcrate_no_stddefault_read_exact {
() => {
// Module: crate::no_std
// Provides: {"default_read_exact"}
// Dependencies: {}
fn default_read_exact < R : Read + ? Sized > (this : & mut R , mut buf : & mut [u8]) -> crate :: Result < () > { while ! buf . is_empty () { match this . read (buf) { Ok (0) => break , Ok (n) => { buf = & mut buf [n ..] ; } Err (Error :: Interrupted) => { } Err (e) => return Err (e) , } } if ! buf . is_empty () { Err (Error :: Eof) } else { Ok (()) } }
};
}
