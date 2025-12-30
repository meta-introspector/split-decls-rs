// Generated macro for impl_255 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_permsimpl_255 {
() => {
// Module: crate::borrow_tracker::tree_borrows::perms
// Provides: {"impl_255"}
// Dependencies: {}
impl PermissionPriv { # [doc = " Check if `self` can be the initial state of a pointer."] fn is_initial (& self) -> bool { matches ! (self , ReservedFrz { conflicted : false } | Frozen | ReservedIM | Cell) } # [doc = " Reject `ReservedIM` that cannot exist in the presence of a protector."] # [cfg (test)] fn compatible_with_protector (& self) -> bool { ! matches ! (self , ReservedIM | Cell) } # [doc = " See `foreign_access_skipping.rs`. Computes the SIFA of a permission."] fn strongest_idempotent_foreign_access (& self , prot : bool) -> IdempotentForeignAccess { match self { Cell => IdempotentForeignAccess :: Write , ReservedFrz { conflicted } if prot && ! conflicted => IdempotentForeignAccess :: None , ReservedFrz { .. } => IdempotentForeignAccess :: Read , ReservedIM if prot => unreachable ! ("Protected ReservedIM should not exist!") , ReservedIM => IdempotentForeignAccess :: Write , Unique => IdempotentForeignAccess :: None , Frozen => IdempotentForeignAccess :: Read , Disabled => IdempotentForeignAccess :: Write , } } }
};
}
