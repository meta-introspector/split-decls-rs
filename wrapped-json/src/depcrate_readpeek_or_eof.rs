// Generated macro for peek_or_eof (function)
macro_rules! Depcrate_readpeek_or_eof {
() => {
// Module: crate::read
// Provides: {"peek_or_eof"}
// Dependencies: {}
fn peek_or_eof < 'de , R > (read : & mut R) -> Result < u8 > where R : ? Sized + Read < 'de > , { match tri ! (read . peek ()) { Some (b) => Ok (b) , None => error (read , ErrorCode :: EofWhileParsingString) , } }
};
}
