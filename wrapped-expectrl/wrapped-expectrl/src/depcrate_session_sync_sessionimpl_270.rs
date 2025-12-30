// Generated macro for impl_270 (impl)
macro_rules! Depcrate_session_sync_sessionimpl_270 {
() => {
// Module: crate::session::sync_session
// Provides: {"impl_270"}
// Dependencies: {}
impl < P , S > Session < P , S > { # [doc = " Set the pty session's expect timeout."] pub fn set_expect_timeout (& mut self , expect_timeout : Option < Duration >) { self . expect_timeout = expect_timeout ; } # [doc = " Set a expect algorithm to be either gready or lazy."] # [doc = ""] # [doc = " Default algorithm is gready."] # [doc = ""] # [doc = " See [Session::expect]."] pub fn set_expect_lazy (& mut self , lazy : bool) { self . expect_lazy = lazy ; } # [doc = " Get a reference to original stream."] pub fn get_stream (& self) -> & S { self . stream . as_ref () } # [doc = " Get a mut reference to original stream."] pub fn get_stream_mut (& mut self) -> & mut S { self . stream . as_mut () } # [doc = " Get a reference to a process running program."] pub fn get_process (& self) -> & P { & self . proc } # [doc = " Get a mut reference to a process running program."] pub fn get_process_mut (& mut self) -> & mut P { & mut self . proc } }
};
}
