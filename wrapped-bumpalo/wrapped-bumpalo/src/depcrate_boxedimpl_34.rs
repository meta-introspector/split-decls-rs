// Generated macro for impl_34 (impl)
macro_rules! Depcrate_boxedimpl_34 {
() => {
// Module: crate::boxed
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'a , F : ? Sized + Future + Unpin > Future for Box < 'a , F > { type Output = F :: Output ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { F :: poll (Pin :: new (& mut * self) , cx) } }
};
}
