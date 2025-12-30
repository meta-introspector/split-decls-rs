// Generated macro for impl_157 (impl)
macro_rules! Depcrate_boxedimpl_157 {
() => {
// Module: crate::boxed
// Provides: {"impl_157"}
// Dependencies: {}
# [unstable (feature = "coroutine_trait" , issue = "43122")] impl < G : ? Sized + Coroutine < R > , R , A : Allocator > Coroutine < R > for Pin < Box < G , A > > where A : 'static , { type Yield = G :: Yield ; type Return = G :: Return ; fn resume (mut self : Pin < & mut Self > , arg : R) -> CoroutineState < Self :: Yield , Self :: Return > { G :: resume ((* self) . as_mut () , arg) } }
};
}
