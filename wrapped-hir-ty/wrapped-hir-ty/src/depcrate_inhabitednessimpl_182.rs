// Generated macro for impl_182 (impl)
macro_rules! Depcrate_inhabitednessimpl_182 {
() => {
// Module: crate::inhabitedness
// Provides: {"impl_182"}
// Dependencies: {}
impl TypeVisitor < Interner > for UninhabitedFrom < '_ > { type BreakTy = VisiblyUninhabited ; fn as_dyn (& mut self) -> & mut dyn TypeVisitor < Interner , BreakTy = VisiblyUninhabited > { self } fn visit_ty (& mut self , ty : & Ty , outer_binder : DebruijnIndex ,) -> ControlFlow < VisiblyUninhabited > { if self . recursive_ty . contains (ty) || self . max_depth == 0 { return CONTINUE_OPAQUELY_INHABITED ; } self . recursive_ty . insert (ty . clone ()) ; self . max_depth -= 1 ; let r = match ty . kind (Interner) { TyKind :: Adt (adt , subst) => self . visit_adt (adt . 0 , subst) , TyKind :: Never => BREAK_VISIBLY_UNINHABITED , TyKind :: Tuple (..) => ty . super_visit_with (self , outer_binder) , TyKind :: Array (item_ty , len) => match try_const_usize (self . db , len) { Some (0) | None => CONTINUE_OPAQUELY_INHABITED , Some (1 ..) => item_ty . super_visit_with (self , outer_binder) , } , TyKind :: Alias (AliasTy :: Projection (projection)) => { let normalized = self . db . normalize_projection (projection . clone () , self . env . clone ()) ; self . visit_ty (& normalized , outer_binder) } _ => CONTINUE_OPAQUELY_INHABITED , } ; self . recursive_ty . remove (ty) ; self . max_depth += 1 ; r } fn interner (& self) -> Interner { Interner } }
};
}
