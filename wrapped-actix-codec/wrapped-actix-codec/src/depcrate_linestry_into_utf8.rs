// Generated macro for try_into_utf8 (function)
macro_rules! Depcrate_linestry_into_utf8 {
() => {
// Module: crate::lines
// Provides: {"try_into_utf8"}
// Dependencies: {}
fn try_into_utf8 (buf : Bytes) -> io :: Result < Option < String > > { String :: from_utf8 (buf . to_vec ()) . map_err (| err | io :: Error :: new (io :: ErrorKind :: InvalidData , err)) . map (Some) }
};
}
