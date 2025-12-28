macro_rules! deps {
    () => {
        ImplTraitInTraitCollector!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < 'tcx , E > TypeFolder < TyCtxt < 'tcx > > for ImplTraitInTraitCollector < '_ , 'tcx , E > where E : 'tcx , { fn cx (& self) -> TyCtxt < 'tcx > { self . ocx . infcx . tcx } fn fold_ty (& mut self , ty : Ty < 'tcx >) -> Ty < 'tcx > { if let ty :: Alias (ty :: Projection , proj) = ty . kind () && self . cx () . is_impl_trait_in_trait (proj . def_id) { if let Some ((ty , _)) = self . types . get (& proj . def_id) { return * ty ; } if proj . args . has_escaping_bound_vars () { bug ! ("FIXME(RPITIT): error here") ; } let infer_ty = self . ocx . infcx . next_ty_var (self . span) ; self . types . insert (proj . def_id , (infer_ty , proj . args)) ; for (pred , pred_span) in self . cx () . explicit_item_bounds (proj . def_id) . iter_instantiated_copied (self . cx () , proj . args) { let pred = pred . fold_with (self) ; let pred = self . ocx . normalize (& ObligationCause :: misc (self . span , self . body_id) , self . param_env , pred ,) ; self . ocx . register_obligation (traits :: Obligation :: new (self . cx () , ObligationCause :: new (self . span , self . body_id , ObligationCauseCode :: WhereClause (proj . def_id , pred_span) ,) , self . param_env , pred ,)) ; } infer_ty } else { ty . super_fold_with (self) } } }
    };
}

impl_46!();