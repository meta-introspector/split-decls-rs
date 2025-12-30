// Generated macro for SHA512_256 (static)
macro_rules! Depcrate_digestSHA512_256 {
() => {
// Module: crate::digest
// Provides: {"SHA512_256"}
// Dependencies: {}
# [doc = " SHA-512/256 as specified in [FIPS 180-4]."] # [doc = ""] # [doc = " This is *not* the same as just truncating the output of SHA-512, as"] # [doc = " SHA-512/256 has its own initial state distinct from SHA-512's initial"] # [doc = " state."] # [doc = ""] # [doc = " [FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf"] pub static SHA512_256 : Algorithm = Algorithm { output_len : OutputLen :: _256 , chaining_len : SHA512_OUTPUT_LEN , block_len : SHA512_BLOCK_LEN , block_data_order : dynstate :: sha512_block_data_order , initial_state : DynInitialState :: new64 ([Wrapping (0x22312194fc2bf72c) , Wrapping (0x9f555fa3c84c64c2) , Wrapping (0x2393b86b6f53b151) , Wrapping (0x963877195940eabd) , Wrapping (0x96283ee2a88effe3) , Wrapping (0xbe5e1e2553863992) , Wrapping (0x2b0199fc2c85b8aa) , Wrapping (0x0eb72ddc81c52ca2) ,]) , id : AlgorithmID :: SHA512_256 , } ;
};
}
