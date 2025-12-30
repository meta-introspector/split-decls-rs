// Generated macro for impl_38 (impl)
macro_rules! Depcrate_contextimpl_38 {
() => {
// Module: crate::context
// Provides: {"impl_38"}
// Dependencies: {}
impl < C , E > Debug for ContextError < C , E > where C : Display , E : Debug , { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("Error") . field ("context" , & Quoted (& self . context)) . field ("source" , & self . error) . finish () } }
};
}
