// Generated macro for impl_12 (impl)
macro_rules! Depcrate_accessimpl_12 {
() => {
// Module: crate::access
// Provides: {"impl_12"}
// Dependencies: {}
impl < T , A : Access < T > + ? Sized , P : Deref < Target = A > > Access < T > for P { type Guard = A :: Guard ; fn load (& self) -> Self :: Guard { self . deref () . load () } }
};
}
