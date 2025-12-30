// Generated macro for impl_61 (impl)
macro_rules! Depcrate_processimpl_61 {
() => {
// Module: crate::process
// Provides: {"impl_61"}
// Dependencies: {}
impl fmt :: Debug for Process { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("PipeReader") . field ("pty_output" , & (self . output . 0)) . field ("pty_output(ptr)" , & (self . output . 0 as * const c_void)) . field ("pty_input" , & (self . input . 0)) . field ("pty_input(ptr)" , & (self . input . 0 as * const c_void)) . finish_non_exhaustive () } }
};
}
