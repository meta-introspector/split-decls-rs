// Generated macro for impl_268 (impl)
macro_rules! Depcrate_ioimpl_268 {
() => {
// Module: crate::io
// Provides: {"impl_268"}
// Dependencies: {}
impl < W : fmt :: Debug > fmt :: Debug for BufWriter < W > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("BufWriter") . field ("writer" , & self . inner) . field ("buf" , & self . buf) . finish () } }
};
}
