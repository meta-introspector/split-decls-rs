// Generated macro for ItemInvalidationCause (enum)
macro_rules! Depcrate_borrow_tracker_stacked_borrowsItemInvalidationCause {
() => {
// Module: crate::borrow_tracker::stacked_borrows
// Provides: {"ItemInvalidationCause"}
// Dependencies: {}
# [doc = " Determines whether an item was invalidated by a conflicting access, or by deallocation."] # [derive (Copy , Clone , Debug)] enum ItemInvalidationCause { Conflict , Dealloc , }
};
}
