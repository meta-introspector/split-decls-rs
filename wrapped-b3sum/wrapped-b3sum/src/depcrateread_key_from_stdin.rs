// Generated macro for read_key_from_stdin (function)
macro_rules! Depcrateread_key_from_stdin {
() => {
// Module: crate
// Provides: {"read_key_from_stdin"}
// Dependencies: {}
fn read_key_from_stdin () -> anyhow :: Result < [u8 ; blake3 :: KEY_LEN] > { let mut bytes = Vec :: with_capacity (blake3 :: KEY_LEN + 1) ; let n = std :: io :: stdin () . lock () . take (blake3 :: KEY_LEN as u64 + 1) . read_to_end (& mut bytes) ? ; if n < blake3 :: KEY_LEN { bail ! ("expected {} key bytes from stdin, found {}" , blake3 :: KEY_LEN , n ,) } else if n > blake3 :: KEY_LEN { bail ! ("read more than {} key bytes from stdin" , blake3 :: KEY_LEN) } else { Ok (bytes [.. blake3 :: KEY_LEN] . try_into () . unwrap ()) } }
};
}
