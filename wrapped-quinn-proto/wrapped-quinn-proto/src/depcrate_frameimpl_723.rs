// Generated macro for impl_723 (impl)
macro_rules! Depcrate_frameimpl_723 {
() => {
// Module: crate::frame
// Provides: {"impl_723"}
// Dependencies: {}
impl fmt :: Display for ConnectionClose { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . error_code . fmt (f) ? ; if ! self . reason . as_ref () . is_empty () { f . write_str (": ") ? ; f . write_str (& String :: from_utf8_lossy (& self . reason)) ? ; } Ok (()) } }
};
}
