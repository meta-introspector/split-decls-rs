// Generated macro for impl_32 (impl)
macro_rules! Depcrate_infer_autoderefimpl_32 {
() => {
// Module: crate::infer::autoderef
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'db , Ctx , Steps > Iterator for GeneralAutoderef < 'db , Ctx , Steps > where Ctx : AutoderefCtx < 'db > , Steps : TrackAutoderefSteps < 'db > , { type Item = (Ty < 'db > , usize) ; fn next (& mut self) -> Option < Self :: Item > { debug ! ("autoderef: steps={:?}, cur_ty={:?}" , self . state . steps , self . state . cur_ty) ; if self . state . at_start { self . state . at_start = false ; debug ! ("autoderef stage #0 is {:?}" , self . state . cur_ty) ; return Some ((self . state . cur_ty , 0)) ; } if self . state . steps . len () >= AUTODEREF_RECURSION_LIMIT { self . state . reached_recursion_limit = true ; return None ; } if self . state . cur_ty . is_ty_var () { return None ; } let (kind , new_ty) = if let Some (ty) = self . state . cur_ty . builtin_deref (self . include_raw_pointers) { debug_assert_eq ! (ty , self . infcx () . resolve_vars_if_possible (ty)) ; if let TyKind :: Alias (..) = ty . kind () { let (normalized_ty , obligations) = structurally_normalize_ty (self . infcx () , self . env () . env , ty) ? ; self . state . obligations . extend (obligations) ; (AutoderefKind :: Builtin , normalized_ty) } else { (AutoderefKind :: Builtin , ty) } } else if let Some (ty) = self . overloaded_deref_ty (self . state . cur_ty) { (AutoderefKind :: Overloaded , ty) } else { return None ; } ; self . state . steps . push (self . state . cur_ty , kind) ; debug ! ("autoderef stage #{:?} is {:?} from {:?}" , self . step_count () , new_ty , (self . state . cur_ty , kind)) ; self . state . cur_ty = new_ty ; Some ((self . state . cur_ty , self . step_count ())) } }
};
}
