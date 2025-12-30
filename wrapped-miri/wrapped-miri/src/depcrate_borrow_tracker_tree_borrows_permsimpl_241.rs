// Generated macro for impl_241 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrows_permsimpl_241 {
() => {
// Module: crate::borrow_tracker::tree_borrows::perms
// Provides: {"impl_241"}
// Dependencies: {}
impl PartialOrd for PermissionPriv { # [doc = " PermissionPriv is ordered by the reflexive transitive closure of"] # [doc = " `Reserved(conflicted=false) < Reserved(conflicted=true) < Active < Frozen < Disabled`."] # [doc = " `Reserved` that have incompatible `ty_is_freeze` are incomparable to each other."] # [doc = " This ordering matches the reachability by transitions, as asserted by the exhaustive test"] # [doc = " `permissionpriv_partialord_is_reachability`."] fn partial_cmp (& self , other : & Self) -> Option < Ordering > { use Ordering :: * ; Some (match (self , other) { (a , b) if a == b => Equal , (ReservedIM , ReservedFrz { .. }) | (ReservedFrz { .. } , ReservedIM) | (Cell , _) | (_ , Cell) => return None , (Disabled , _) => Greater , (_ , Disabled) => Less , (Frozen , _) => Greater , (_ , Frozen) => Less , (Active , _) => Greater , (_ , Active) => Less , (ReservedIM , ReservedIM) => Equal , (ReservedFrz { conflicted : c1 } , ReservedFrz { conflicted : c2 }) => { c1 . cmp (c2) } }) } }
};
}
