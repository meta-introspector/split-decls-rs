// Generated macro for impl_52 (impl)
macro_rules! Depcrate_lz_lz_encoderimpl_52 {
() => {
// Module: crate::lz::lz_encoder
// Provides: {"impl_52"}
// Dependencies: {}
impl MatchFind for MatchFinders { fn find_matches (& mut self , encoder : & mut LzEncoderData , matches : & mut Matches) { match self { MatchFinders :: Hc4 (m) => m . find_matches (encoder , matches) , MatchFinders :: Bt4 (m) => m . find_matches (encoder , matches) , } } fn skip (& mut self , encoder : & mut LzEncoderData , len : usize) { match self { MatchFinders :: Hc4 (m) => m . skip (encoder , len) , MatchFinders :: Bt4 (m) => m . skip (encoder , len) , } } }
};
}
