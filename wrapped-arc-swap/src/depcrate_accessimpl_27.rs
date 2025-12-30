// Generated macro for impl_27 (impl)
macro_rules! Depcrate_accessimpl_27 {
() => {
// Module: crate::access
// Provides: {"impl_27"}
// Dependencies: {}
impl < T , D > Access < T > for AccessConvert < D > where D : Deref , D :: Target : DynAccess < T > , { type Guard = DynGuard < T > ; fn load (& self) -> Self :: Guard { self . 0 . load () } }
};
}
