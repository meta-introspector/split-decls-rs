// Generated macro for impl_1350 (impl)
macro_rules! Depcrate_resultimpl_1350 {
() => {
// Module: crate::result
// Provides: {"impl_1350"}
// Dependencies: {}
impl < T : fmt :: Debug , E : fmt :: Debug > statics :: MapFn < T > for WrapOk < T , E > { type Output = Result < T , E > ; fn apply (& self , t : T) -> Result < T , E > { Ok (t) } }
};
}
