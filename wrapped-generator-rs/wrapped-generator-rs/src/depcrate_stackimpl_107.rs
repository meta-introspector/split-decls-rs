// Generated macro for impl_107 (impl)
macro_rules! Depcrate_stackimpl_107 {
() => {
// Module: crate::stack
// Provides: {"impl_107"}
// Dependencies: {}
impl fmt :: Debug for Stack { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { let offset = self . get_offset () ; write ! (f , "Stack<{:?}, Offset={}>" , self . buf , unsafe { * offset }) } }
};
}
