// Generated macro for impl_2199 (impl)
macro_rules! Depcrate_io_chainimpl_2199 {
() => {
// Module: crate::io::chain
// Provides: {"impl_2199"}
// Dependencies: {}
impl < T , U > fmt :: Debug for Chain < T , U > where T : fmt :: Debug , U : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Chain") . field ("t" , & self . first) . field ("u" , & self . second) . field ("done_first" , & self . done_first) . finish () } }
};
}
