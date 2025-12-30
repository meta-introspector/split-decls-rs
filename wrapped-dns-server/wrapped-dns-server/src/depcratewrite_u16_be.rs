// Generated macro for write_u16_be (function)
macro_rules! Depcratewrite_u16_be {
() => {
// Module: crate
// Provides: {"write_u16_be"}
// Dependencies: {}
fn write_u16_be < const N : usize > (out : & mut FixedBuf < N > , value : u16) -> Result < () , DnsError > { let bytes : [u8 ; 2] = value . to_be_bytes () ; out . write_bytes (& bytes) . map_err (| _ | DnsError :: ResponseBufferFull) ? ; Ok (()) }
};
}
