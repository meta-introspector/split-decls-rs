// Generated macro for impl_21 (impl)
macro_rules! Depcrate_export_implimpl_21 {
() => {
// Module: crate::export_impl
// Provides: {"impl_21"}
// Dependencies: {}
impl fmt :: Display for DisplayDuration { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let nanos = self . 0 . as_nanos () ; if nanos > 100_000_000 { write ! (f , "{:.3}s" , self . 0 . as_secs_f64 ()) } else if nanos > 1_000_000 { write ! (f , "{:.3}ms" , (nanos as f64) / 1e6) } else if nanos > 1_000 { write ! (f , "{:.3}µs" , (nanos as f64) / 1e3) } else { write ! (f , "{nanos}ns") } } }
};
}
