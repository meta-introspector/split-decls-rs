// Generated macro for read_exact (function)
macro_rules! Depcrateread_exact {
() => {
// Module: crate
// Provides: {"read_exact"}
// Dependencies: {}
fn read_exact < const N : usize , const M : usize > (buf : & mut FixedBuf < N >) -> Result < [u8 ; M] , DnsError > { let mut result = [0_u8 ; M] ; buf . try_read_exact (& mut result) . ok_or (DnsError :: Truncated) ? ; Ok (result) }
};
}
