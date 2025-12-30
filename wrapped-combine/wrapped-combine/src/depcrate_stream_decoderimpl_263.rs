// Generated macro for impl_263 (impl)
macro_rules! Depcrate_stream_decoderimpl_263 {
() => {
// Module: crate::stream::decoder
// Provides: {"impl_263"}
// Dependencies: {}
impl < S , P , C > Decoder < S , P , C > where C : , { # [doc (hidden)] pub fn __before_parse < R > (& mut self , mut reader : R) -> io :: Result < () > where R : Read , C : crate :: stream :: buf_reader :: CombineSyncRead < R > , { if self . buffer . extend_buf_sync (& mut reader) ? == 0 { self . end_of_input = true ; } Ok (()) } }
};
}
