// Generated macro for SHA384 (static)
macro_rules! Depcrate_digestSHA384 {
() => {
// Module: crate::digest
// Provides: {"SHA384"}
// Dependencies: {}
# [doc = " SHA-384 as specified in [FIPS 180-4]."] # [doc = ""] # [doc = " [FIPS 180-4]: http://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.180-4.pdf"] pub static SHA384 : Algorithm = Algorithm { output_len : OutputLen :: _384 , chaining_len : SHA512_OUTPUT_LEN , block_len : SHA512_BLOCK_LEN , block_data_order : dynstate :: sha512_block_data_order , initial_state : DynInitialState :: new64 ([Wrapping (0xcbbb9d5dc1059ed8) , Wrapping (0x629a292a367cd507) , Wrapping (0x9159015a3070dd17) , Wrapping (0x152fecd8f70e5939) , Wrapping (0x67332667ffc00b31) , Wrapping (0x8eb44a8768581511) , Wrapping (0xdb0c2e0d64f98fa7) , Wrapping (0x47b5481dbefa4fa4) ,]) , id : AlgorithmID :: SHA384 , } ;
};
}
