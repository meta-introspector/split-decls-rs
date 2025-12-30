// Generated macro for impl_823 (impl)
macro_rules! Depcrate_extensionsimpl_823 {
() => {
// Module: crate::extensions
// Provides: {"impl_823"}
// Dependencies: {}
impl < 'a > DataContext < 'a > for ExtensionContext < 'a > { fn data < D : Any + Send + Sync > (& self) -> Result < & 'a D > { ExtensionContext :: data :: < D > (self) } fn data_unchecked < D : Any + Send + Sync > (& self) -> & 'a D { ExtensionContext :: data_unchecked :: < D > (self) } fn data_opt < D : Any + Send + Sync > (& self) -> Option < & 'a D > { ExtensionContext :: data_opt :: < D > (self) } }
};
}
