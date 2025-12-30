// Generated macro for impl_335 (impl)
macro_rules! Depcrate_ioimpl_335 {
() => {
// Module: crate::io
// Provides: {"impl_335"}
// Dependencies: {}
impl < R1 : fmt :: Debug , R2 : fmt :: Debug > fmt :: Debug for Chain < R1 , R2 > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Chain") . field ("r1" , & self . first) . field ("r2" , & self . second) . finish () } }
};
}
