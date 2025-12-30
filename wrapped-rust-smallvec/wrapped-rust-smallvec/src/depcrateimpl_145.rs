// Generated macro for impl_145 (impl)
macro_rules! Depcrateimpl_145 {
() => {
// Module: crate
// Provides: {"impl_145"}
// Dependencies: {}
impl < T > Drop for DropGuard < T > { # [inline] fn drop (& mut self) { unsafe { core :: ptr :: slice_from_raw_parts_mut (self . ptr , self . len) . drop_in_place () ; } } }
};
}
