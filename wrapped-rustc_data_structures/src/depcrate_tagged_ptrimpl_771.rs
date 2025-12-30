// Generated macro for impl_771 (impl)
macro_rules! Depcrate_tagged_ptrimpl_771 {
() => {
// Module: crate::tagged_ptr
// Provides: {"impl_771"}
// Dependencies: {}
impl < P , T > Deref for TaggedRef < '_ , P , T > where P : Aligned + ? Sized , T : Tag , { type Target = P ; # [inline] fn deref (& self) -> & Self :: Target { self . pointer () } }
};
}
