// Generated macro for impl_462 (impl)
macro_rules! Depcrate_ffiimpl_462 {
() => {
// Module: crate::ffi
// Provides: {"impl_462"}
// Dependencies: {}
impl log :: Log for Logger { fn enabled (& self , _metadata : & log :: Metadata) -> bool { true } fn log (& self , record : & log :: Record) { let line = format ! ("{}: {}\0" , record . target () , record . args ()) ; (self . cb) (line . as_ptr () , self . argp . load (atomic :: Ordering :: Relaxed)) ; } fn flush (& self) { } }
};
}
