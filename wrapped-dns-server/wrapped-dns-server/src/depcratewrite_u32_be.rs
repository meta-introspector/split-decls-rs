// Generated macro for write_u32_be (function)
macro_rules! Depcratewrite_u32_be {
() => {
// Module: crate
// Provides: {"write_u32_be"}
// Dependencies: {}
fn write_u32_be < const N : usize > (out : & mut FixedBuf < N > , value : u32) -> Result < () , DnsError > { let bytes : [u8 ; 4] = value . to_be_bytes () ; out . write_bytes (& bytes) . map_err (| _ | DnsError :: ResponseBufferFull) ? ; Ok (()) }
};
}
