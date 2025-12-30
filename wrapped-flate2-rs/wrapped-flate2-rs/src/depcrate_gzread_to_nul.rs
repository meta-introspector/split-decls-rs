// Generated macro for read_to_nul (function)
macro_rules! Depcrate_gzread_to_nul {
() => {
// Module: crate::gz
// Provides: {"read_to_nul"}
// Dependencies: {}
fn read_to_nul < R : BufRead > (r : & mut R , buffer : & mut Vec < u8 >) -> Result < () > { let mut bytes = r . bytes () ; loop { match bytes . next () . transpose () ? { Some (0) => return Ok (()) , Some (_) if buffer . len () == MAX_HEADER_BUF => { return Err (Error :: new (ErrorKind :: InvalidInput , "gzip header field too long" ,)) ; } Some (byte) => { buffer . push (byte) ; } None => { return Err (ErrorKind :: UnexpectedEof . into ()) ; } } } }
};
}
