// Generated macro for next_or_eof (function)
macro_rules! Depcrate_readnext_or_eof {
() => {
// Module: crate::read
// Provides: {"next_or_eof"}
// Dependencies: {}
fn next_or_eof < 'de , R > (read : & mut R) -> Result < u8 > where R : ? Sized + Read < 'de > , { match tri ! (read . next ()) { Some (b) => Ok (b) , None => error (read , ErrorCode :: EofWhileParsingString) , } }
};
}
