// Generated macro for impl_3292 (impl)
macro_rules! Depcrate_large_stack_framesimpl_3292 {
() => {
// Module: crate::large_stack_frames
// Provides: {"impl_3292"}
// Dependencies: {}
impl fmt :: Display for Space { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Space :: Used (1) => write ! (f , "1 byte") , Space :: Used (n) => write ! (f , "{n} bytes") , Space :: Overflow => write ! (f , "over 2⁶⁴-1 bytes") , } } }
};
}
