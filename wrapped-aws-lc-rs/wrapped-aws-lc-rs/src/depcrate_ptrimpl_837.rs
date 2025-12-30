// Generated macro for impl_837 (impl)
macro_rules! Depcrate_ptrimpl_837 {
() => {
// Module: crate::ptr
// Provides: {"impl_837"}
// Dependencies: {}
impl < P : Pointer > From < DetachablePointer < P > > for ManagedPointer < P > { # [inline] fn from (mut dptr : DetachablePointer < P >) -> Self { match dptr . pointer . take () { Some (pointer) => ManagedPointer { pointer } , None => { unreachable ! () } } } }
};
}
