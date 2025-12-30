// Generated macro for impl_138 (impl)
macro_rules! Depcrate_writersimpl_138 {
() => {
// Module: crate::writers
// Provides: {"impl_138"}
// Dependencies: {}
impl fmt :: Display for BacktraceOmited { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . 0 { write ! (f , "Backtrace omitted. Run with RUST_BACKTRACE=1 environment variable to display it.") ? ; } else { write ! (f , "Run with COLORBT_SHOW_HIDDEN=1 environment variable to disable frame filtering.") ? ; } Ok (()) } }
};
}
