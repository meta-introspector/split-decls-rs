// Generated macro for impl_70 (impl)
macro_rules! Depcrate_borrow_setimpl_70 {
() => {
// Module: crate::borrow_set
// Provides: {"impl_70"}
// Dependencies: {}
impl LocalsStateAtExit { fn build < 'tcx > (locals_are_invalidated_at_exit : bool , body : & Body < 'tcx > , move_data : & MoveData < 'tcx > ,) -> Self { struct HasStorageDead (DenseBitSet < Local >) ; impl < 'tcx > Visitor < 'tcx > for HasStorageDead { fn visit_local (& mut self , local : Local , ctx : PlaceContext , _ : Location) { if ctx == PlaceContext :: NonUse (NonUseContext :: StorageDead) { self . 0 . insert (local) ; } } } if locals_are_invalidated_at_exit { LocalsStateAtExit :: AllAreInvalidated } else { let mut has_storage_dead = HasStorageDead (DenseBitSet :: new_empty (body . local_decls . len ())) ; has_storage_dead . visit_body (body) ; let mut has_storage_dead_or_moved = has_storage_dead . 0 ; for move_out in & move_data . moves { has_storage_dead_or_moved . insert (move_data . base_local (move_out . path)) ; } LocalsStateAtExit :: SomeAreInvalidated { has_storage_dead_or_moved } } } }
};
}
