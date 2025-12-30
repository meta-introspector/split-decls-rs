// Generated macro for read_u8 (function)
macro_rules! Depcrateread_u8 {
() => {
// Module: crate
// Provides: {"read_u8"}
// Dependencies: {}
fn read_u8 < const N : usize > (buf : & mut FixedBuf < N >) -> Result < u8 , DnsError > { buf . try_read_byte () . ok_or (DnsError :: Truncated) }
};
}
