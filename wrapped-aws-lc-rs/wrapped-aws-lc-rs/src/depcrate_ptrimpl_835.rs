// Generated macro for impl_835 (impl)
macro_rules! Depcrate_ptrimpl_835 {
() => {
// Module: crate::ptr
// Provides: {"impl_835"}
// Dependencies: {}
impl < P : Pointer > Deref for DetachablePointer < P > { type Target = P ; # [inline] fn deref (& self) -> & Self :: Target { match & self . pointer { Some (pointer) => pointer , None => { unreachable ! () } } } }
};
}
