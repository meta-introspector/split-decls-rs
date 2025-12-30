// Generated macro for impl_254 (impl)
macro_rules! Depcrate_session_async_sessionimpl_254 {
() => {
// Module: crate::session::async_session
// Provides: {"impl_254"}
// Dependencies: {}
impl < S > Stream < S > { # [doc = " Creates an async IO stream."] fn new (stream : S) -> Self { Self { stream : BufferedStream :: new (stream) , expect_timeout : Some (Duration :: from_millis (10000)) , expect_lazy : false , } } # [doc = " Returns a reference to original stream."] fn as_ref (& self) -> & S { & self . stream . stream } # [doc = " Returns a mut reference to original stream."] fn as_mut (& mut self) -> & mut S { & mut self . stream . stream } # [doc = " Set the pty session's expect timeout."] fn set_expect_timeout (& mut self , expect_timeout : Option < Duration >) { self . expect_timeout = expect_timeout ; } # [doc = " Save a bytes in inner buffer."] # [doc = " They'll be pushed to the end of the buffer."] fn keep (& mut self , buf : & [u8]) { self . stream . keep (buf) ; } # [doc = " Get an inner buffer."] fn get_available (& mut self) -> & [u8] { self . stream . buffer () } # [doc = " Returns an inner IO stream."] fn into_inner (self) -> S { self . stream . stream } }
};
}
