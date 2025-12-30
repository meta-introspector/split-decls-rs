// Generated macro for impl_156 (impl)
macro_rules! Depcrate_boxedimpl_156 {
() => {
// Module: crate::boxed
// Provides: {"impl_156"}
// Dependencies: {}
# [unstable (feature = "coroutine_trait" , issue = "43122")] impl < G : ? Sized + Coroutine < R > + Unpin , R , A : Allocator > Coroutine < R > for Box < G , A > { type Yield = G :: Yield ; type Return = G :: Return ; fn resume (mut self : Pin < & mut Self > , arg : R) -> CoroutineState < Self :: Yield , Self :: Return > { G :: resume (Pin :: new (& mut * self) , arg) } }
};
}
