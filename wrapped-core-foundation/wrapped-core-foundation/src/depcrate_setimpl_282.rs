// Generated macro for impl_282 (impl)
macro_rules! Depcrate_setimpl_282 {
() => {
// Module: crate::set
// Provides: {"impl_282"}
// Dependencies: {}
impl < T > CFSet < T > { # [doc = " Get the number of elements in the `CFSet`."] pub fn len (& self) -> usize { unsafe { CFSetGetCount (self . 0) as usize } } # [doc = " Returns `true` if the set contains no elements."] pub fn is_empty (& self) -> bool { self . len () == 0 } }
};
}
