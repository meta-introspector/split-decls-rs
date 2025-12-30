// Generated macro for impl_358 (impl)
macro_rules! Depcrate_borrow_tracker_tree_borrowsimpl_358 {
() => {
// Module: crate::borrow_tracker::tree_borrows
// Provides: {"impl_358"}
// Dependencies: {}
impl < 'tcx > NewPermission { # [doc = " Determine NewPermission of the reference/Box from the type of the pointee."] # [doc = ""] # [doc = " A `ref_mutability` of `None` indicates a `Box` type."] fn new (pointee : Ty < 'tcx > , ref_mutability : Option < Mutability > , retag_kind : RetagKind , cx : & crate :: MiriInterpCx < 'tcx > ,) -> Option < Self > { let ty_is_unpin = pointee . is_unpin (* cx . tcx , cx . typing_env ()) ; let ty_is_freeze = pointee . is_freeze (* cx . tcx , cx . typing_env ()) ; let is_protected = retag_kind == RetagKind :: FnEntry ; if matches ! (ref_mutability , Some (Mutability :: Mut) | None if ! ty_is_unpin) { return None ; } let freeze_perm = match ref_mutability { Some (Mutability :: Not) => Permission :: new_frozen () , _ => Permission :: new_reserved_frz () , } ; let nonfreeze_perm = match ref_mutability { Some (Mutability :: Not) => Permission :: new_cell () , _ if is_protected => Permission :: new_reserved_frz () , _ => Permission :: new_reserved_im () , } ; let initial_access = | perm : & Permission | ! perm . is_cell () ; Some (NewPermission { freeze_perm , freeze_access : initial_access (& freeze_perm) , nonfreeze_perm , nonfreeze_access : initial_access (& nonfreeze_perm) , outside_perm : if ty_is_freeze { freeze_perm } else { nonfreeze_perm } , protector : is_protected . then_some (if ref_mutability . is_some () { ProtectorKind :: StrongProtector } else { ProtectorKind :: WeakProtector }) , }) } }
};
}
