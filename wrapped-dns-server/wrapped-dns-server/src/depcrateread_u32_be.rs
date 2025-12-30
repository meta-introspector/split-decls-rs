// Generated macro for read_u32_be (function)
macro_rules! Depcrateread_u32_be {
() => {
// Module: crate
// Provides: {"read_u32_be"}
// Dependencies: {}
fn read_u32_be < const N : usize > (buf : & mut FixedBuf < N >) -> Result < u32 , DnsError > { let bytes : [u8 ; 4] = read_exact (buf) ? ; Ok (u32 :: from_be_bytes ([bytes [0] , bytes [1] , bytes [2] , bytes [3]])) }
};
}
