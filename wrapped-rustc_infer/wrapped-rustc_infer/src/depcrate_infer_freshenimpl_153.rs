// Generated macro for impl_153 (impl)
macro_rules! Depcrate_infer_freshenimpl_153 {
() => {
// Module: crate::infer::freshen
// Provides: {"impl_153"}
// Dependencies: {}
impl < 'a , 'tcx > TypeFreshener < 'a , 'tcx > { # [inline (never)] fn fold_infer_ty (& mut self , v : ty :: InferTy) -> Option < Ty < 'tcx > > { match v { ty :: TyVar (v) => { let mut inner = self . infcx . inner . borrow_mut () ; let input = inner . type_variables () . probe (v) . known () . ok_or_else (| | ty :: TyVar (inner . type_variables () . root_var (v))) ; drop (inner) ; Some (self . freshen_ty (input , | n | Ty :: new_fresh (self . infcx . tcx , n))) } ty :: IntVar (v) => { let mut inner = self . infcx . inner . borrow_mut () ; let value = inner . int_unification_table () . probe_value (v) ; let input = match value { ty :: IntVarValue :: IntType (ty) => Ok (Ty :: new_int (self . infcx . tcx , ty)) , ty :: IntVarValue :: UintType (ty) => Ok (Ty :: new_uint (self . infcx . tcx , ty)) , ty :: IntVarValue :: Unknown => { Err (ty :: IntVar (inner . int_unification_table () . find (v))) } } ; drop (inner) ; Some (self . freshen_ty (input , | n | Ty :: new_fresh_int (self . infcx . tcx , n))) } ty :: FloatVar (v) => { let mut inner = self . infcx . inner . borrow_mut () ; let value = inner . float_unification_table () . probe_value (v) ; let input = match value { ty :: FloatVarValue :: Known (ty) => Ok (Ty :: new_float (self . infcx . tcx , ty)) , ty :: FloatVarValue :: Unknown => { Err (ty :: FloatVar (inner . float_unification_table () . find (v))) } } ; drop (inner) ; Some (self . freshen_ty (input , | n | Ty :: new_fresh_float (self . infcx . tcx , n))) } ty :: FreshTy (ct) | ty :: FreshIntTy (ct) | ty :: FreshFloatTy (ct) => { if ct >= self . ty_freshen_count { bug ! ("Encountered a freshend type with id {} \
                          but our counter is only at {}" , ct , self . ty_freshen_count) ; } None } } } }
};
}
