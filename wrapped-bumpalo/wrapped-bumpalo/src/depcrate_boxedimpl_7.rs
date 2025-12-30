// Generated macro for impl_7 (impl)
macro_rules! Depcrate_boxedimpl_7 {
() => {
// Module: crate::boxed
// Provides: {"impl_7"}
// Dependencies: {}
impl < 'a , T : ? Sized > Drop for Box < 'a , T > { fn drop (& mut self) { unsafe { core :: ptr :: drop_in_place (self . 0) ; } } }
};
}
