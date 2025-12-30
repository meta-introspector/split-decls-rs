// Generated macro for impl_27 (impl)
macro_rules! Depcrate_framedimpl_27 {
() => {
// Module: crate::framed
// Provides: {"impl_27"}
// Dependencies: {}
impl < T , U > fmt :: Debug for Framed < T , U > where T : fmt :: Debug , U : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Framed") . field ("io" , & self . io) . field ("codec" , & self . codec) . finish () } }
};
}
