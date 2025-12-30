// Generated macro for impl_165 (impl)
macro_rules! Depcrate_context_implimpl_165 {
() => {
// Module: crate::context_impl
// Provides: {"impl_165"}
// Dependencies: {}
impl < A > fmt :: Debug for ContextParts < A > where A : Actor , A :: Context : AsyncContext < A > , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("ContextParts") . field ("flags" , & self . flags) . finish () } }
};
}
