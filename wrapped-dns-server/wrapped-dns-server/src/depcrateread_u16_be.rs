// Generated macro for read_u16_be (function)
macro_rules! Depcrateread_u16_be {
() => {
// Module: crate
// Provides: {"read_u16_be"}
// Dependencies: {}
fn read_u16_be < const N : usize > (buf : & mut FixedBuf < N >) -> Result < u16 , DnsError > { let bytes : [u8 ; 2] = read_exact (buf) ? ; Ok (u16 :: from_be_bytes ([bytes [0] , bytes [1]])) }
};
}
