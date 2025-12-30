// Generated macro for impl_129 (impl)
macro_rules! Depcrate_engine_general_purpose_decodeimpl_129 {
() => {
// Module: crate::engine::general_purpose::decode
// Provides: {"impl_129"}
// Dependencies: {}
impl GeneralPurposeEstimate { pub (crate) fn new (encoded_len : usize) -> Self { let rem = encoded_len % 4 ; Self { rem , conservative_decoded_len : (encoded_len / 4 + usize :: from (rem > 0)) * 3 , } } }
};
}
