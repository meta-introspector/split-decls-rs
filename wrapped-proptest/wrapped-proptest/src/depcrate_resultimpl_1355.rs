// Generated macro for impl_1355 (impl)
macro_rules! Depcrate_resultimpl_1355 {
() => {
// Module: crate::result
// Provides: {"impl_1355"}
// Dependencies: {}
impl < T : fmt :: Debug , E : fmt :: Debug > statics :: MapFn < E > for WrapErr < T , E > { type Output = Result < T , E > ; fn apply (& self , e : E) -> Result < T , E > { Err (e) } }
};
}
