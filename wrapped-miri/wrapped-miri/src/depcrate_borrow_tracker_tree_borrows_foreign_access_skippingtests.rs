// Generated macro for tests (module)
macro_rules! Depcrate_borrow_tracker_tree_borrows_foreign_access_skippingtests {
() => {
// Module: crate::borrow_tracker::tree_borrows::foreign_access_skipping
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: IdempotentForeignAccess ; # [test] fn test_order () { assert ! (IdempotentForeignAccess :: None < IdempotentForeignAccess :: Read) ; assert ! (IdempotentForeignAccess :: Read < IdempotentForeignAccess :: Write) ; } }
};
}
