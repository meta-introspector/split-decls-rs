// Generated macro for impl_335 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_wildcardimpl_335 {
() => {
// Module: crate::borrow_tracker::tree_borrows::wildcard
// Provides: {"impl_335"}
// Dependencies: {}
impl Debug for WildcardState { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("WildcardState") . field ("child_r/w" , & (self . child_reads , self . child_writes)) . field ("foreign" , & self . max_foreign_access) . field ("exposed_as" , & self . exposed_as) . finish () } }
};
}
