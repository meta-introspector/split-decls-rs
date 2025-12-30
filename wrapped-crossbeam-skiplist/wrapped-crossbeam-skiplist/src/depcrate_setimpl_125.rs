// Generated macro for impl_125 (impl)
macro_rules! Depcrate_setimpl_125 {
() => {
// Module: crate::set
// Provides: {"impl_125"}
// Dependencies: {}
impl < 'a , T > Entry < 'a , T > { fn new (inner : map :: Entry < 'a , T , () >) -> Self { Self { inner } } # [doc = " Returns a reference to the value."] pub fn value (& self) -> & 'a T { self . inner . key () } # [doc = " Returns `true` if the entry is removed from the set."] pub fn is_removed (& self) -> bool { self . inner . is_removed () } }
};
}
