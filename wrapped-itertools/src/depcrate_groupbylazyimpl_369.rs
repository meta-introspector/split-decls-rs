// Generated macro for impl_369 (impl)
macro_rules! Depcrate_groupbylazyimpl_369 {
() => {
// Module: crate::groupbylazy
// Provides: {"impl_369"}
// Dependencies: {}
impl < 'a , K , I , F > Drop for Group < 'a , K , I , F > where I : Iterator , I :: Item : 'a , { fn drop (& mut self) { self . parent . drop_group (self . index) ; } }
};
}
