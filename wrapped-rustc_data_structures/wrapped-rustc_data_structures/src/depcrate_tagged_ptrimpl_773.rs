// Generated macro for impl_773 (impl)
macro_rules! Depcrate_tagged_ptrimpl_773 {
() => {
// Module: crate::tagged_ptr
// Provides: {"impl_773"}
// Dependencies: {}
impl < P , T > PartialEq for TaggedRef < '_ , P , T > where P : Aligned + ? Sized , T : Tag , { # [inline] # [allow (ambiguous_wide_pointer_comparisons)] fn eq (& self , other : & Self) -> bool { self . packed == other . packed } }
};
}
