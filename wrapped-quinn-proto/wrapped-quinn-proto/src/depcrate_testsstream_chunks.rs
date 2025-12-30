// Generated macro for stream_chunks (function)
macro_rules! Depcrate_testsstream_chunks {
() => {
// Module: crate::tests
// Provides: {"stream_chunks"}
// Dependencies: {}
fn stream_chunks (mut recv : RecvStream) -> Vec < u8 > { let mut buf = Vec :: new () ; let mut chunks = recv . read (true) . unwrap () ; while let Ok (Some (chunk)) = chunks . next (usize :: MAX) { buf . extend (chunk . bytes) ; } let _ = chunks . finalize () ; buf }
};
}
