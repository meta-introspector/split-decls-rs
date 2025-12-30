// Generated macro for FromStream (trait)
macro_rules! Depcrate_socketFromStream {
() => {
// Module: crate::socket
// Provides: {"FromStream"}
// Dependencies: {}
# [doc = " Helper trait for converting a Mio stream into a Tokio stream."] pub trait FromStream : Sized { # [doc = " Creates stream from a `mio` stream."] fn from_mio (sock : MioStream) -> io :: Result < Self > ; }
};
}
