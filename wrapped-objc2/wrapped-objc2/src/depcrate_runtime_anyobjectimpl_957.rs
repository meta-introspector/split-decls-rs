// Generated macro for impl_957 (impl)
macro_rules! Depcrate_runtime_anyobjectimpl_957 {
() => {
// Module: crate::runtime::anyobject
// Provides: {"impl_957"}
// Dependencies: {}
impl fmt :: Debug for AnyObject { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let ptr : * const Self = self ; write ! (f , "<{}: {:p}>" , self . class () , ptr) } }
};
}
