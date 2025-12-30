// Generated macro for impl_363 (impl)
macro_rules! Depcrate_groupbylazyimpl_363 {
() => {
// Module: crate::groupbylazy
// Provides: {"impl_363"}
// Dependencies: {}
impl < K , I , F > ChunkBy < K , I , F > where I : Iterator , { # [doc = " `client`: Index of group that requests next element"] fn step (& self , client : usize) -> Option < I :: Item > where F : FnMut (& I :: Item) -> K , K : PartialEq , { self . inner . borrow_mut () . step (client) } # [doc = " `client`: Index of group"] fn drop_group (& self , client : usize) { self . inner . borrow_mut () . drop_group (client) ; } }
};
}
