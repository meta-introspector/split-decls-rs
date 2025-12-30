// Generated macro for impl_371 (impl)
macro_rules! Depcrate_groupbylazyimpl_371 {
() => {
// Module: crate::groupbylazy
// Provides: {"impl_371"}
// Dependencies: {}
impl < 'a , K , I , F > Iterator for Group < 'a , K , I , F > where I : Iterator , I :: Item : 'a , F : FnMut (& I :: Item) -> K , K : PartialEq , { type Item = I :: Item ; # [inline] fn next (& mut self) -> Option < Self :: Item > { if let elt @ Some (..) = self . first . take () { return elt ; } self . parent . step (self . index) } }
};
}
