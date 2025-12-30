// Generated macro for impl_266 (impl)
macro_rules! Depcrate_stream_decoderimpl_266 {
() => {
// Module: crate::stream::decoder
// Provides: {"impl_266"}
// Dependencies: {}
# [cfg (feature = "tokio")] impl < S , P , C > Decoder < S , P , C > { # [doc (hidden)] pub async fn __before_parse_tokio < R > (& mut self , mut reader : Pin < & mut R >) -> io :: Result < () > where R : tokio_dep :: io :: AsyncRead , C : crate :: stream :: buf_reader :: CombineRead < R , dyn tokio_dep :: io :: AsyncRead > , { let copied = crate :: future_ext :: poll_fn (| cx | self . buffer . poll_extend_buf (cx , reader . as_mut ())) . await ? ; if copied == 0 { self . end_of_input = true ; } Ok (()) } }
};
}
