// Generated macro for impl_32 (impl)
macro_rules! Depcrate_segimpl_32 {
() => {
// Module: crate::seg
// Provides: {"impl_32"}
// Dependencies: {}
impl Parser for Text { type Item = str ; type Error = core :: str :: Utf8Error ; fn parse < 'a > (& mut self , bytes : & 'a mut [u8]) -> Result < & 'a str , Self :: Error > { if bytes . len () <= self . stored { return Ok ("") ; } bytes [.. self . stored] . clone_from_slice (& self . buffer [.. self . stored]) ; Ok (match core :: str :: from_utf8 (bytes) { Ok (s) => { self . stored = 0 ; s } Err (e) => { let valid_len = e . valid_up_to () ; let invalid_len = bytes . len () - valid_len ; if invalid_len > self . buffer . len () { return Err (e) ; } self . buffer [.. invalid_len] . clone_from_slice (& bytes [valid_len ..]) ; self . stored = invalid_len ; core :: str :: from_utf8 (& bytes [.. valid_len]) . unwrap () } }) } fn saved (& self) -> usize { self . stored } }
};
}
