// Generated macro for impl_417 (impl)
macro_rules! Depcrate_mirimpl_417 {
() => {
// Module: crate::mir
// Provides: {"impl_417"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for V < '_ > { fn visit_place (& mut self , place : & Place < 'tcx > , ctx : PlaceContext , loc : Location) { if loc . block == self . location . block && loc . statement_index <= self . location . statement_index { return ; } let local = place . local ; for (i , self_local) in self . locals . iter () . enumerate () { if local == * self_local { if ! matches ! (ctx , PlaceContext :: MutatingUse (MutatingUseContext :: Drop) | PlaceContext :: NonUse (_)) { self . results [i] . local_use_locs . push (loc) ; } if matches ! (ctx , PlaceContext :: NonMutatingUse (NonMutatingUseContext :: Move | NonMutatingUseContext :: Inspect) | PlaceContext :: MutatingUse (MutatingUseContext :: Borrow)) { self . results [i] . local_consume_or_mutate_locs . push (loc) ; } } } } }
};
}
