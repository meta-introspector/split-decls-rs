// Generated macro for orderly_close (function)
macro_rules! Depcrateorderly_close {
() => {
// Module: crate
// Provides: {"orderly_close"}
// Dependencies: {}
fn orderly_close (conn : & mut net :: TcpStream) { conn . shutdown (net :: Shutdown :: Write) . unwrap () ; let mut buf = [0u8 ; 32] ; while let Ok (p @ 1 ..) = conn . peek (& mut buf) { let _ = conn . read (& mut buf [.. p]) . unwrap () ; } let _ = conn . shutdown (net :: Shutdown :: Read) ; }
};
}
