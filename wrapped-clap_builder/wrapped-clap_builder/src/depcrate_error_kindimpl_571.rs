// Generated macro for impl_571 (impl)
macro_rules! Depcrate_error_kindimpl_571 {
() => {
// Module: crate::error::kind
// Provides: {"impl_571"}
// Dependencies: {}
impl std :: fmt :: Display for ErrorKind { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { self . as_str () . unwrap_or_default () . fmt (f) } }
};
}
