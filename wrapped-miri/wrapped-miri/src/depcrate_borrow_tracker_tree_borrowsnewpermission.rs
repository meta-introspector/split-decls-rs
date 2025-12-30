// Generated macro for NewPermission (struct)
macro_rules! Depcrate_borrow_tracker_tree_borrowsNewPermission {
() => {
// Module: crate::borrow_tracker::tree_borrows
// Provides: {"NewPermission"}
// Dependencies: {}
# [doc = " Policy for a new borrow."] # [derive (Debug , Clone , Copy)] pub struct NewPermission { # [doc = " Permission for the frozen part of the range."] freeze_perm : Permission , # [doc = " Whether a read access should be performed on the frozen part on a retag."] freeze_access : bool , # [doc = " Permission for the non-frozen part of the range."] nonfreeze_perm : Permission , # [doc = " Whether a read access should be performed on the non-frozen"] # [doc = " part on a retag."] nonfreeze_access : bool , # [doc = " Permission for memory outside the range."] outside_perm : Permission , # [doc = " Whether this pointer is part of the arguments of a function call."] # [doc = " `protector` is `Some(_)` for all pointers marked `noalias`."] protector : Option < ProtectorKind > , }
};
}
