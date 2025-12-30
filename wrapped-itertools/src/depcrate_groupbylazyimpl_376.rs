// Generated macro for impl_376 (impl)
macro_rules! Depcrate_groupbylazyimpl_376 {
() => {
// Module: crate::groupbylazy
// Provides: {"impl_376"}
// Dependencies: {}
impl < I > IntoChunks < I > where I : Iterator , { # [doc = " `client`: Index of chunk that requests next element"] fn step (& self , client : usize) -> Option < I :: Item > { self . inner . borrow_mut () . step (client) } # [doc = " `client`: Index of chunk"] fn drop_group (& self , client : usize) { self . inner . borrow_mut () . drop_group (client) ; } }
};
}
