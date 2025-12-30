// Generated macro for impl_158 (impl)
macro_rules! Depcrate_boxedimpl_158 {
() => {
// Module: crate::boxed
// Provides: {"impl_158"}
// Dependencies: {}
# [stable (feature = "futures_api" , since = "1.36.0")] impl < F : ? Sized + Future + Unpin , A : Allocator > Future for Box < F , A > { type Output = F :: Output ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { F :: poll (Pin :: new (& mut * self) , cx) } }
};
}
