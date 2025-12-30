// Generated macro for impl_28 (impl)
macro_rules! Depcrate_contextimpl_28 {
() => {
// Module: crate::context
// Provides: {"impl_28"}
// Dependencies: {}
impl < D , E > Debug for ContextError < D , E > where D : Display , E : Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Error") . field ("msg" , & Quoted (& self . msg)) . field ("source" , & self . error) . finish () } }
};
}
