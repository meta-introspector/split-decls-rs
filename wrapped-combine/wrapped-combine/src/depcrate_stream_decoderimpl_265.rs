// Generated macro for impl_265 (impl)
macro_rules! Depcrate_stream_decoderimpl_265 {
() => {
// Module: crate::stream::decoder
// Provides: {"impl_265"}
// Dependencies: {}
# [cfg (feature = "tokio-03")] impl < S , P , C > Decoder < S , P , C > { # [doc (hidden)] pub async fn __before_parse_tokio_03 < R > (& mut self , mut reader : Pin < & mut R >) -> io :: Result < () > where R : tokio_03_dep :: io :: AsyncRead , C : crate :: stream :: buf_reader :: CombineRead < R , dyn tokio_03_dep :: io :: AsyncRead > , { let copied = crate :: future_ext :: poll_fn (| cx | self . buffer . poll_extend_buf (cx , reader . as_mut ())) . await ? ; if copied == 0 { self . end_of_input = true ; } Ok (()) } }
};
}
