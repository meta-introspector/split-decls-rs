// Generated macro for impl_86 (impl)
macro_rules! Depcrate_frontendimpl_86 {
() => {
// Module: crate::frontend
// Provides: {"impl_86"}
// Dependencies: {}
impl < B : PatternBackend > core :: fmt :: Debug for Pattern < B > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Pattern") . field ("_backend" , & self . _backend) . field ("store" , & & self . store) . finish () } }
};
}
