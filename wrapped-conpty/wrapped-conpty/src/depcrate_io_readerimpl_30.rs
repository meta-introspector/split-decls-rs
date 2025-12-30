// Generated macro for impl_30 (impl)
macro_rules! Depcrate_io_readerimpl_30 {
() => {
// Module: crate::io::reader
// Provides: {"impl_30"}
// Dependencies: {}
impl fmt :: Debug for PipeReader { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("PipeReader") . field ("handle" , & (self . handle . 0)) . field ("handle(ptr)" , & (self . handle . 0 as * const c_void)) . finish () } }
};
}
