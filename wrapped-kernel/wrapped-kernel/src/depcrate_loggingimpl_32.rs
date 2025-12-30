// Generated macro for impl_32 (impl)
macro_rules! Depcrate_loggingimpl_32 {
() => {
// Module: crate::logging
// Provides: {"impl_32"}
// Dependencies: {}
impl fmt :: Display for Microseconds { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let seconds = self . 0 / 1_000_000 ; let microseconds = self . 0 % 1_000_000 ; write ! (f , "{seconds:5}.{microseconds:06}") } }
};
}
