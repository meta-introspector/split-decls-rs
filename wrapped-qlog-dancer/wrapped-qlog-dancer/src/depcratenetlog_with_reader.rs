// Generated macro for netlog_with_reader (function)
macro_rules! Depcratenetlog_with_reader {
() => {
// Module: crate
// Provides: {"netlog_with_reader"}
// Dependencies: {}
pub fn netlog_with_reader < R : std :: io :: BufRead > (reader : & mut R ,) -> Result < netlog :: constants :: Constants , Box < dyn Error > > { let mut buf = Vec :: < u8 > :: new () ; let len = reader . read_until (b'\n' , & mut buf) . unwrap () ; buf [len - 2] = b'}' ; let res : Result < netlog :: constants :: ConstantsLine , serde_json :: Error > = serde_json :: from_slice (& buf) ; match res { Ok (mut line) => { line . constants . populate_id_keyed () ; Ok (line . constants) } , Err (e) => { error ! ("Error deserializing: {}" , e) ; Err (e . into ()) } , } }
};
}
