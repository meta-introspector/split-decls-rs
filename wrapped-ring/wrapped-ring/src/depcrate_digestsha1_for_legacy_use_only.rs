// Generated macro for SHA1_FOR_LEGACY_USE_ONLY (static)
macro_rules! Depcrate_digestSHA1_FOR_LEGACY_USE_ONLY {
() => {
// Module: crate::digest
// Provides: {"SHA1_FOR_LEGACY_USE_ONLY"}
// Dependencies: {}
# [doc = " SHA-1 as specified in [FIPS 180-4]. Deprecated."] # [doc = ""] # [doc = " [FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf"] pub static SHA1_FOR_LEGACY_USE_ONLY : Algorithm = Algorithm { output_len : sha1 :: OUTPUT_LEN , chaining_len : sha1 :: CHAINING_LEN , block_len : sha1 :: BLOCK_LEN , block_data_order : dynstate :: sha1_block_data_order , initial_state : DynInitialState :: new32 ([Wrapping (0x67452301u32) , Wrapping (0xefcdab89u32) , Wrapping (0x98badcfeu32) , Wrapping (0x10325476u32) , Wrapping (0xc3d2e1f0u32) , Wrapping (0) , Wrapping (0) , Wrapping (0) ,]) , id : AlgorithmID :: SHA1 , } ;
};
}
