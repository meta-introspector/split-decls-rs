// Generated macro for impl_159 (impl)
macro_rules! Depcrate_boxedimpl_159 {
() => {
// Module: crate::boxed
// Provides: {"impl_159"}
// Dependencies: {}
# [stable (feature = "box_error" , since = "1.8.0")] impl < E : Error > Error for Box < E > { # [allow (deprecated)] fn cause (& self) -> Option < & dyn Error > { Error :: cause (& * * self) } fn source (& self) -> Option < & (dyn Error + 'static) > { Error :: source (& * * self) } fn provide < 'b > (& 'b self , request : & mut error :: Request < 'b >) { Error :: provide (& * * self , request) ; } }
};
}
