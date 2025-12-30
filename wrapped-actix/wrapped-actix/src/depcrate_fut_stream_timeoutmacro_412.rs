// Generated macro for macro_412 (macro)
macro_rules! Depcrate_fut_stream_timeoutmacro_412 {
() => {
// Module: crate::fut::stream::timeout
// Provides: {"macro_412"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`timeout`](super::ActorStreamExt::timeout) method."] # [derive (Debug)] # [must_use = "streams do nothing unless polled"] pub struct Timeout < S > { # [pin] stream : S , dur : Duration , reset_timeout : bool , # [pin] timeout : Sleep , } }
};
}
