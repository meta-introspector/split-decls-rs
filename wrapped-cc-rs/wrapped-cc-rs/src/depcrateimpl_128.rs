// Generated macro for impl_128 (impl)
macro_rules! Depcrateimpl_128 {
() => {
// Module: crate
// Provides: {"impl_128"}
// Dependencies: {}
impl Error { fn new (kind : ErrorKind , message : impl Into < Cow < 'static , str > >) -> Error { Error { kind , message : message . into () , } } }
};
}
