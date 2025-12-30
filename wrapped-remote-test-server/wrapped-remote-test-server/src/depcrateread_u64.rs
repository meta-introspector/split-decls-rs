// Generated macro for read_u64 (function)
macro_rules! Depcrateread_u64 {
() => {
// Module: crate
// Provides: {"read_u64"}
// Dependencies: {}
fn read_u64 (r : & mut dyn Read) -> u64 { let mut len = [0 ; 8] ; t ! (r . read_exact (& mut len)) ; u64 :: from_be_bytes (len) }
};
}
