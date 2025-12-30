// Generated macro for to_std_error (function)
macro_rules! Depcrate_stdto_std_error {
() => {
// Module: crate::std
// Provides: {"to_std_error"}
// Dependencies: {}
# [doc = " Convert a embedded-io error to a [`std::io::Error`]"] pub fn to_std_error < T : embedded_io :: Error > (err : T) -> std :: io :: Error { std :: io :: Error :: new (err . kind () . into () , format ! ("{err:?}")) }
};
}
