// Generated macro for impl_267 (impl)
macro_rules! Depcrate_stream_decoderimpl_267 {
() => {
// Module: crate::stream::decoder
// Provides: {"impl_267"}
// Dependencies: {}
# [cfg (feature = "futures-03")] impl < S , P , C > Decoder < S , P , C > { # [doc (hidden)] pub async fn __before_parse_async < R > (& mut self , reader : Pin < & mut R >) -> io :: Result < () > where R : futures_io_03 :: AsyncRead , C : crate :: stream :: buf_reader :: CombineAsyncRead < R > , { let copied = self . buffer . extend_buf (reader) . await ? ; if copied == 0 { self . end_of_input = true ; } Ok (()) } }
};
}
