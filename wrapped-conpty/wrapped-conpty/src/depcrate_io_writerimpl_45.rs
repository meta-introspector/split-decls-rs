// Generated macro for impl_45 (impl)
macro_rules! Depcrate_io_writerimpl_45 {
() => {
// Module: crate::io::writer
// Provides: {"impl_45"}
// Dependencies: {}
impl fmt :: Debug for PipeWriter { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("PipeWriter") . field ("handle" , & (self . handle . 0)) . field ("handle(ptr)" , & (self . handle . 0 as * const c_void)) . finish () } }
};
}
