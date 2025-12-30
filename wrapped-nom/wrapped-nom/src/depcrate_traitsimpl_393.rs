// Generated macro for impl_393 (impl)
macro_rules! Depcrate_traitsimpl_393 {
() => {
// Module: crate::traits
// Provides: {"impl_393"}
// Dependencies: {}
impl < 'a > Offset for & 'a [u8] { fn offset (& self , second : & Self) -> usize { let fst = self . as_ptr () ; let snd = second . as_ptr () ; snd as usize - fst as usize } }
};
}
