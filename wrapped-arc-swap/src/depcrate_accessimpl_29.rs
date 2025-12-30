// Generated macro for impl_29 (impl)
macro_rules! Depcrate_accessimpl_29 {
() => {
// Module: crate::access
// Provides: {"impl_29"}
// Dependencies: {}
impl < G , F , T , R > Deref for MapGuard < G , F , T , R > where G : Deref < Target = T > , F : Fn (& T) -> & R , { type Target = R ; fn deref (& self) -> & R { (self . projection) (& self . guard) } }
};
}
