// Generated macro for verify_mode (function)
macro_rules! Depcrate_plumbing_mainverify_mode {
() => {
// Module: crate::plumbing::main
// Provides: {"verify_mode"}
// Dependencies: {}
fn verify_mode (decode : bool , re_encode : bool) -> verify :: Mode { match (decode , re_encode) { (true , false) => verify :: Mode :: HashCrc32Decode , (_ , true) => verify :: Mode :: HashCrc32DecodeEncode , (false , false) => verify :: Mode :: HashCrc32 , } }
};
}
