// Generated macro for write_hex_output (function)
macro_rules! Depcratewrite_hex_output {
() => {
// Module: crate
// Provides: {"write_hex_output"}
// Dependencies: {}
fn write_hex_output (mut output : blake3 :: OutputReader , args : & Args) -> anyhow :: Result < () > { let mut len = args . len () ; let mut block = [0 ; blake3 :: BLOCK_LEN] ; while len > 0 { output . fill (& mut block) ; let hex_str = hex :: encode (& block [..]) ; let take_bytes = cmp :: min (len , block . len () as u64) ; print ! ("{}" , & hex_str [.. 2 * take_bytes as usize]) ; len -= take_bytes ; } Ok (()) }
};
}
