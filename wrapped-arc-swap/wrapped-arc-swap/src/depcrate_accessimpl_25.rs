// Generated macro for impl_25 (impl)
macro_rules! Depcrate_accessimpl_25 {
() => {
// Module: crate::access
// Provides: {"impl_25"}
// Dependencies: {}
impl < T , A > DynAccess < T > for A where A : Access < T > , A :: Guard : 'static , { fn load (& self) -> DynGuard < T > { DynGuard (Box :: new (Access :: load (self))) } }
};
}
