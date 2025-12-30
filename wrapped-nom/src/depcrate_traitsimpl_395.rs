// Generated macro for impl_395 (impl)
macro_rules! Depcrate_traitsimpl_395 {
() => {
// Module: crate::traits
// Provides: {"impl_395"}
// Dependencies: {}
impl < 'a > Offset for & 'a str { fn offset (& self , second : & Self) -> usize { let fst = self . as_ptr () ; let snd = second . as_ptr () ; snd as usize - fst as usize } }
};
}
