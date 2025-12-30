// Generated macro for skip_spaces (function)
macro_rules! Depcrateskip_spaces {
() => {
// Module: crate
// Provides: {"skip_spaces"}
// Dependencies: {}
# [inline] fn skip_spaces (bytes : & mut Bytes < '_ >) -> Result < () > { loop { let b = bytes . peek () ; match b { Some (b' ') => { unsafe { bytes . bump () } ; } Some (..) => { bytes . slice () ; return Ok (Status :: Complete (())) ; } None => return Ok (Status :: Partial) , } } }
};
}
