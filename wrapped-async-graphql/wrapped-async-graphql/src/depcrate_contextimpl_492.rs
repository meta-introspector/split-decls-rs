// Generated macro for impl_492 (impl)
macro_rules! Depcrate_contextimpl_492 {
() => {
// Module: crate::context
// Provides: {"impl_492"}
// Dependencies: {}
impl < 'a , T > DataContext < 'a > for ContextBase < 'a , T > { fn data < D : Any + Send + Sync > (& self) -> Result < & 'a D > { ContextBase :: data :: < D > (self) } fn data_unchecked < D : Any + Send + Sync > (& self) -> & 'a D { ContextBase :: data_unchecked :: < D > (self) } fn data_opt < D : Any + Send + Sync > (& self) -> Option < & 'a D > { ContextBase :: data_opt :: < D > (self) } }
};
}
