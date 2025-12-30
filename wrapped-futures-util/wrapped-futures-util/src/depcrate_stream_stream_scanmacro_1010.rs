// Generated macro for macro_1010 (macro)
macro_rules! Depcrate_stream_stream_scanmacro_1010 {
() => {
// Module: crate::stream::stream::scan
// Provides: {"macro_1010"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`scan`](super::StreamExt::scan) method."] # [must_use = "streams do nothing unless polled"] pub struct Scan < St : Stream , S , Fut , F > { # [pin] stream : St , f : F , # [pin] state : UnfoldState < S , Fut >, } }
};
}
