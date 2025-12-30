// Generated macro for as_str (function)
macro_rules! Depcrate_readas_str {
() => {
// Module: crate::read
// Provides: {"as_str"}
// Dependencies: {}
fn as_str < 'de , 's , R : Read < 'de > > (read : & R , slice : & 's [u8]) -> Result < & 's str > { str :: from_utf8 (slice) . or_else (| _ | error (read , ErrorCode :: InvalidUnicodeCodePoint)) }
};
}
