// Generated macro for impl_216 (impl)
macro_rules! Depcrate_utilimpl_216 {
() => {
// Module: crate::util
// Provides: {"impl_216"}
// Dependencies: {}
impl < 'a , R : RawRwLockDowngrade > RwLockWriteGuardDetached < 'a , R > { # [doc = " # Safety"] # [doc = ""] # [doc = " The associated data must not mut mutated after downgrading"] pub (crate) unsafe fn downgrade (self) -> RwLockReadGuardDetached < 'a , R > { let this = ManuallyDrop :: new (self) ; unsafe { this . lock . downgrade () } RwLockReadGuardDetached { lock : this . lock , _marker : this . _marker , } } }
};
}
