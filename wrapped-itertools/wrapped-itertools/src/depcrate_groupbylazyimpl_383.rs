// Generated macro for impl_383 (impl)
macro_rules! Depcrate_groupbylazyimpl_383 {
() => {
// Module: crate::groupbylazy
// Provides: {"impl_383"}
// Dependencies: {}
impl < 'a , I > Iterator for Chunk < 'a , I > where I : Iterator , I :: Item : 'a , { type Item = I :: Item ; # [inline] fn next (& mut self) -> Option < Self :: Item > { if let elt @ Some (..) = self . first . take () { return elt ; } self . parent . step (self . index) } }
};
}
