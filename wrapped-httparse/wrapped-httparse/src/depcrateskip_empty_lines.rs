// Generated macro for skip_empty_lines (function)
macro_rules! Depcrateskip_empty_lines {
() => {
// Module: crate
// Provides: {"skip_empty_lines"}
// Dependencies: {}
# [inline] fn skip_empty_lines (bytes : & mut Bytes < '_ >) -> Result < () > { loop { let b = bytes . peek () ; match b { Some (b'\r') => { unsafe { bytes . bump () } ; expect ! (bytes . next () == b'\n' => Err (Error :: NewLine)) ; } Some (b'\n') => { unsafe { bytes . bump () ; } } Some (..) => { bytes . slice () ; return Ok (Status :: Complete (())) ; } None => return Ok (Status :: Partial) , } } }
};
}
