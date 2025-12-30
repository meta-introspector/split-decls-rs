// Generated macro for impl_32 (impl)
macro_rules! Depcrate_atomicimpl_32 {
() => {
// Module: crate::atomic
// Provides: {"impl_32"}
// Dependencies: {}
impl < T > Array < T > { fn layout (len : usize) -> Layout { Layout :: new :: < Self > () . extend (Layout :: array :: < MaybeUninit < T > > (len) . unwrap ()) . unwrap () . 0 . pad_to_align () } }
};
}
