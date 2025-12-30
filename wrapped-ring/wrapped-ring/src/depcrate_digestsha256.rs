// Generated macro for SHA256 (static)
macro_rules! Depcrate_digestSHA256 {
() => {
// Module: crate::digest
// Provides: {"SHA256"}
// Dependencies: {}
# [doc = " SHA-256 as specified in [FIPS 180-4]."] # [doc = ""] # [doc = " [FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf"] pub static SHA256 : Algorithm = Algorithm { output_len : OutputLen :: _256 , chaining_len : SHA256_OUTPUT_LEN , block_len : SHA256_BLOCK_LEN , block_data_order : dynstate :: sha256_block_data_order , initial_state : DynInitialState :: new32 ([Wrapping (0x6a09e667u32) , Wrapping (0xbb67ae85u32) , Wrapping (0x3c6ef372u32) , Wrapping (0xa54ff53au32) , Wrapping (0x510e527fu32) , Wrapping (0x9b05688cu32) , Wrapping (0x1f83d9abu32) , Wrapping (0x5be0cd19u32) ,]) , id : AlgorithmID :: SHA256 , } ;
};
}
