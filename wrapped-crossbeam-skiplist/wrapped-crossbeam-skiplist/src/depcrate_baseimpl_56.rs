// Generated macro for impl_56 (impl)
macro_rules! Depcrate_baseimpl_56 {
() => {
// Module: crate::base
// Provides: {"impl_56"}
// Dependencies: {}
impl < 'a , K : 'a , V : 'a > RefIter < 'a , K , V > { # [doc = " Decrements the reference count of `RefEntry` owned by the iterator."] pub fn drop_impl (& mut self , guard : & Guard) { self . parent . check_guard (guard) ; if let Some (e) = self . head . take () { unsafe { e . node . decrement (guard) } ; } if let Some (e) = self . tail . take () { unsafe { e . node . decrement (guard) } ; } } }
};
}
