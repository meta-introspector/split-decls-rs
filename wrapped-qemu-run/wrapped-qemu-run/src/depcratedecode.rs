// Generated macro for decode (function)
macro_rules! Depcratedecode {
() => {
// Module: crate
// Provides: {"decode"}
// Dependencies: {}
fn decode (decoder : & mut dyn StreamDecoder) -> Result < () , DecodeError > { loop { match decoder . decode () { Ok (frame) => { println ! ("{}" , frame . display (true)) } Err (DecodeError :: UnexpectedEof) => return Ok (()) , Err (DecodeError :: Malformed) => { eprintln ! ("failed to decode defmt data") ; return Err (DecodeError :: Malformed) ; } } } }
};
}
