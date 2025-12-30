// Generated macro for write_bytes (function)
macro_rules! Depcratewrite_bytes {
() => {
// Module: crate
// Provides: {"write_bytes"}
// Dependencies: {}
fn write_bytes < const N : usize > (out : & mut FixedBuf < N > , bytes : & [u8]) -> Result < () , DnsError > { out . write_bytes (bytes) . map_err (| _ | DnsError :: ResponseBufferFull) ? ; Ok (()) }
};
}
