// Generated macro for impl_728 (impl)
macro_rules! Depcrate_frameimpl_728 {
() => {
// Module: crate::frame
// Provides: {"impl_728"}
// Dependencies: {}
impl fmt :: Display for ApplicationClose { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if ! self . reason . as_ref () . is_empty () { f . write_str (& String :: from_utf8_lossy (& self . reason)) ? ; f . write_str (" (code ") ? ; self . error_code . fmt (f) ? ; f . write_str (")") ? ; } else { self . error_code . fmt (f) ? ; } Ok (()) } }
};
}
