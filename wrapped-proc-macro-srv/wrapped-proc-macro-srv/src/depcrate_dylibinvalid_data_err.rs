// Generated macro for invalid_data_err (function)
macro_rules! Depcrate_dylibinvalid_data_err {
() => {
// Module: crate::dylib
// Provides: {"invalid_data_err"}
// Dependencies: {}
fn invalid_data_err (e : impl Into < Box < dyn std :: error :: Error + Send + Sync > >) -> io :: Error { io :: Error :: new (io :: ErrorKind :: InvalidData , e) }
};
}
