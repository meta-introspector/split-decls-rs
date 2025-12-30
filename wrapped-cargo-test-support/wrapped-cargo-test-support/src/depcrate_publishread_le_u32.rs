// Generated macro for read_le_u32 (function)
macro_rules! Depcrate_publishread_le_u32 {
() => {
// Module: crate::publish
// Provides: {"read_le_u32"}
// Dependencies: {}
fn read_le_u32 < R > (mut reader : R) -> io :: Result < u32 > where R : Read , { let mut buf = [0 ; 4] ; reader . read_exact (& mut buf) ? ; Ok (u32 :: from_le_bytes (buf)) }
};
}
