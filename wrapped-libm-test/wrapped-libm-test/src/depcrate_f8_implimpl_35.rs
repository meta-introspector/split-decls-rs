// Generated macro for impl_35 (impl)
macro_rules! Depcrate_f8_implimpl_35 {
() => {
// Module: crate::f8_impl
// Provides: {"impl_35"}
// Dependencies: {}
impl fmt :: Binary for f8 { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let v = self . 0 ; write ! (f , "0b{:b}_{:04b}_{:03b}" , v >> 7 , (v & Self :: EXP_MASK) >> Self :: SIG_BITS , v & Self :: SIG_MASK) } }
};
}
