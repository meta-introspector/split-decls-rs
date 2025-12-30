// Generated macro for impl_382 (impl)
macro_rules! Depcrate_groupbylazyimpl_382 {
() => {
// Module: crate::groupbylazy
// Provides: {"impl_382"}
// Dependencies: {}
impl < 'a , I > Drop for Chunk < 'a , I > where I : Iterator , I :: Item : 'a , { fn drop (& mut self) { self . parent . drop_group (self . index) ; } }
};
}
