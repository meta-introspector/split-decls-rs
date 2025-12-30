// Generated macro for impl_80 (impl)
macro_rules! Depcrate_errorimpl_80 {
() => {
// Module: crate::error
// Provides: {"impl_80"}
// Dependencies: {}
impl < 'ast > Visit < 'ast > for SearchImpl { fn visit_type (& mut self , i : & 'ast syn :: Type) { if self . 0 { return ; } if let syn :: Type :: ImplTrait (_) = i { self . 0 = true } visit :: visit_type (self , i) ; } }
};
}
