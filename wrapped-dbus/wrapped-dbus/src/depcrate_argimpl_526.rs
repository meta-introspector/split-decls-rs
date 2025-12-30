// Generated macro for impl_526 (impl)
macro_rules! Depcrate_argimpl_526 {
() => {
// Module: crate::arg
// Provides: {"impl_526"}
// Dependencies: {}
impl fmt :: Display for TypeMismatchError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "D-Bus argument type mismatch at position {}: expected {}, found {}" , self . position , self . expected . as_str () , if self . expected == self . found { "same but still different somehow" } else { self . found . as_str () }) } }
};
}
