// Generated macro for SHA512 (static)
macro_rules! Depcrate_digestSHA512 {
() => {
// Module: crate::digest
// Provides: {"SHA512"}
// Dependencies: {}
# [doc = " SHA-512 as specified in [FIPS 180-4]."] # [doc = ""] # [doc = " [FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf"] pub static SHA512 : Algorithm = Algorithm { output_len : OutputLen :: _512 , chaining_len : SHA512_OUTPUT_LEN , block_len : SHA512_BLOCK_LEN , block_data_order : dynstate :: sha512_block_data_order , initial_state : DynInitialState :: new64 ([Wrapping (0x6a09e667f3bcc908) , Wrapping (0xbb67ae8584caa73b) , Wrapping (0x3c6ef372fe94f82b) , Wrapping (0xa54ff53a5f1d36f1) , Wrapping (0x510e527fade682d1) , Wrapping (0x9b05688c2b3e6c1f) , Wrapping (0x1f83d9abfb41bd6b) , Wrapping (0x5be0cd19137e2179) ,]) , id : AlgorithmID :: SHA512 , } ;
};
}
