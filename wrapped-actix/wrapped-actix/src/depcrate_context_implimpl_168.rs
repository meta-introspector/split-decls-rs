// Generated macro for impl_168 (impl)
macro_rules! Depcrate_context_implimpl_168 {
() => {
// Module: crate::context_impl
// Provides: {"impl_168"}
// Dependencies: {}
impl < A , C > fmt :: Debug for ContextFut < A , C > where C : AsyncContextParts < A > + Unpin , A : Actor < Context = C > , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "ContextFut {{ /* omitted */ }}") } }
};
}
