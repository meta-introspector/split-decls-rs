// Generated macro for impl_392 (impl)
macro_rules! Depcrate_traitsimpl_392 {
() => {
// Module: crate::traits
// Provides: {"impl_392"}
// Dependencies: {}
impl Offset for [u8] { fn offset (& self , second : & Self) -> usize { let fst = self . as_ptr () ; let snd = second . as_ptr () ; snd as usize - fst as usize } }
};
}
