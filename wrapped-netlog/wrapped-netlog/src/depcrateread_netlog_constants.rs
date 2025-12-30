// Generated macro for read_netlog_constants (function)
macro_rules! Depcrateread_netlog_constants {
() => {
// Module: crate
// Provides: {"read_netlog_constants"}
// Dependencies: {}
# [doc = " Read the netlog constants from a netlog file accessed by a BufRead."] pub fn read_netlog_constants < R : BufRead > (reader : & mut R ,) -> Result < Constants , serde_json :: Error > { let mut buf = Vec :: < u8 > :: new () ; let len = reader . read_until (b'\n' , & mut buf) . unwrap () ; buf [len - 2] = b'}' ; let res : Result < ConstantsLine , serde_json :: Error > = serde_json :: from_slice (& buf) ; match res { Ok (mut line) => { line . constants . populate_id_keyed () ; Ok (line . constants) } , Err (e) => { log :: error ! ("Error deserializing constants: {}" , e) ; Err (e) } , } }
};
}
