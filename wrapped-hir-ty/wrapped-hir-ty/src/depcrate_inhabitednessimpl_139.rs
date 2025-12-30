// Generated macro for impl_139 (impl)
macro_rules! Depcrate_inhabitednessimpl_139 {
() => {
// Module: crate::inhabitedness
// Provides: {"impl_139"}
// Dependencies: {}
impl < 'db > TypeVisitor < DbInterner < 'db > > for UninhabitedFrom < '_ , 'db > { type Result = ControlFlow < VisiblyUninhabited > ; fn visit_ty (& mut self , mut ty : Ty < 'db >) -> ControlFlow < VisiblyUninhabited > { if self . recursive_ty . contains (& ty) || self . max_depth == 0 { return CONTINUE_OPAQUELY_INHABITED ; } self . recursive_ty . insert (ty) ; self . max_depth -= 1 ; if matches ! (ty . kind () , TyKind :: Alias (..)) { let mut ocx = ObligationCtxt :: new (self . infcx) ; match ocx . structurally_normalize_ty (& ObligationCause :: dummy () , self . env . env , ty) { Ok (it) => ty = it , Err (_) => return CONTINUE_OPAQUELY_INHABITED , } } let r = match ty . kind () { TyKind :: Adt (adt , subst) => self . visit_adt (adt . def_id () . 0 , subst) , TyKind :: Never => BREAK_VISIBLY_UNINHABITED , TyKind :: Tuple (..) => ty . super_visit_with (self) , TyKind :: Array (item_ty , len) => match try_const_usize (self . infcx . interner . db , len) { Some (0) | None => CONTINUE_OPAQUELY_INHABITED , Some (1 ..) => item_ty . visit_with (self) , } , _ => CONTINUE_OPAQUELY_INHABITED , } ; self . recursive_ty . remove (& ty) ; self . max_depth += 1 ; r } }
};
}
