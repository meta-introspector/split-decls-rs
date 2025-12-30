// Generated macro for NewPermission (enum)
macro_rules! Depcrate_borrow_tracker_stacked_borrowsNewPermission {
() => {
// Module: crate::borrow_tracker::stacked_borrows
// Provides: {"NewPermission"}
// Dependencies: {}
# [doc = " Indicates which permissions to grant to the retagged pointer."] # [derive (Clone , Debug)] enum NewPermission { Uniform { perm : Permission , access : Option < AccessKind > , protector : Option < ProtectorKind > , } , FreezeSensitive { freeze_perm : Permission , freeze_access : Option < AccessKind > , freeze_protector : Option < ProtectorKind > , nonfreeze_perm : Permission , nonfreeze_access : Option < AccessKind > , } , }
};
}
