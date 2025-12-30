// Generated macro for impl_3232 (impl)
macro_rules! Depcrate_large_stack_framesimpl_3232 {
() => {
// Module: crate::large_stack_frames
// Provides: {"impl_3232"}
// Dependencies: {}
impl fmt :: Display for Space { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Space :: Used (1) => write ! (f , "1 byte") , Space :: Used (n) => write ! (f , "{n} bytes") , Space :: Overflow => write ! (f , "over 2⁶⁴-1 bytes") , } } }
};
}
