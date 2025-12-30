// Generated macro for impl_260 (impl)
macro_rules! Depcrate_session_async_sessionimpl_260 {
() => {
// Module: crate::session::async_session
// Provides: {"impl_260"}
// Dependencies: {}
impl < S > BufferedStream < S > { fn new (stream : S) -> Self { Self { stream , buffer : Vec :: new () , length : 0 , } } fn keep (& mut self , buf : & [u8]) { self . buffer . extend (buf) ; self . length += buf . len () ; } fn buffer (& self) -> & [u8] { & self . buffer [.. self . length] } fn get_mut (& mut self) -> & mut S { & mut self . stream } }
};
}
