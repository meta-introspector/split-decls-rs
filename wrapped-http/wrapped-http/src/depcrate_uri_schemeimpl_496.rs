// Generated macro for impl_496 (impl)
macro_rules! Depcrate_uri_schemeimpl_496 {
() => {
// Module: crate::uri::scheme
// Provides: {"impl_496"}
// Dependencies: {}
impl Scheme2 < usize > { fn parse_exact (s : & [u8]) -> Result < Scheme2 < () > , InvalidUri > { match s { b"http" => Ok (Protocol :: Http . into ()) , b"https" => Ok (Protocol :: Https . into ()) , _ => { if s . len () > MAX_SCHEME_LEN { return Err (ErrorKind :: SchemeTooLong . into ()) ; } for & b in s { match SCHEME_CHARS [b as usize] { b':' => { return Err (ErrorKind :: InvalidScheme . into ()) ; } 0 => { return Err (ErrorKind :: InvalidScheme . into ()) ; } _ => { } } } Ok (Scheme2 :: Other (())) } } } pub (super) fn parse (s : & [u8]) -> Result < Scheme2 < usize > , InvalidUri > { if s . len () >= 7 { if s [.. 7] . eq_ignore_ascii_case (b"http://") { return Ok (Protocol :: Http . into ()) ; } } if s . len () >= 8 { if s [.. 8] . eq_ignore_ascii_case (b"https://") { return Ok (Protocol :: Https . into ()) ; } } if s . len () > 3 { for i in 0 .. s . len () { let b = s [i] ; match SCHEME_CHARS [b as usize] { b':' => { if s . len () < i + 3 { break ; } if & s [i + 1 .. i + 3] != b"//" { break ; } if i > MAX_SCHEME_LEN { return Err (ErrorKind :: SchemeTooLong . into ()) ; } return Ok (Scheme2 :: Other (i)) ; } 0 => break , _ => { } } } } Ok (Scheme2 :: None) } }
};
}
