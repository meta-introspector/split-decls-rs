// Generated macro for macro_1611 (macro)
macro_rules! Depcrate_stream_poll_immediatemacro_1611 {
() => {
// Module: crate::stream::poll_immediate
// Provides: {"macro_1611"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [poll_immediate](poll_immediate()) function."] # [doc = ""] # [doc = " It will never return [Poll::Pending](core::task::Poll::Pending)"] # [derive (Debug , Clone)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct PollImmediate < S > { # [pin] stream : Option < S > } }
};
}
