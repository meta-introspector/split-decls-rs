// Generated macro for split (function)
macro_rules! Depcrate_iosplit {
() => {
// Module: crate::io
// Provides: {"split"}
// Dependencies: {}
# [doc = " Splits a stream into [`AsyncRead`] and [`AsyncWrite`] halves."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::{self, Cursor};"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " let stream = Cursor::new(vec![]);"] # [doc = " let (mut reader, mut writer) = io::split(stream);"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub fn split < T > (stream : T) -> (ReadHalf < T > , WriteHalf < T >) where T : AsyncRead + AsyncWrite + Unpin , { let inner = Arc :: new (Mutex :: new (stream)) ; (ReadHalf (inner . clone ()) , WriteHalf (inner)) }
};
}
