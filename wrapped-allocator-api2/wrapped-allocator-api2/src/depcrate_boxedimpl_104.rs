// Generated macro for impl_104 (impl)
macro_rules! Depcrate_boxedimpl_104 {
() => {
// Module: crate::boxed
// Provides: {"impl_104"}
// Dependencies: {}
impl < F : ? Sized + Future + Unpin , A : Allocator > Future for Box < F , A > where A : 'static , { type Output = F :: Output ; # [inline (always)] fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { F :: poll (Pin :: new (& mut * self) , cx) } }
};
}
