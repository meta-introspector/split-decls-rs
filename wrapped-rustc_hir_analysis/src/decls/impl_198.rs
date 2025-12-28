macro_rules! deps {
    () => {
        UncoveredTyParamCollector!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for UncoveredTyParamCollector < '_ , 'tcx > { fn visit_ty (& mut self , ty : Ty < 'tcx >) -> Self :: Result { if ! ty . has_type_flags (ty :: TypeFlags :: HAS_TY_INFER) { return ; } let ty :: Infer (ty :: TyVar (vid)) = * ty . kind () else { return ty . super_visit_with (self) ; } ; let origin = self . infcx . type_var_origin (vid) ; if let Some (def_id) = origin . param_def_id { self . uncovered_params . insert (def_id) ; } } fn visit_const (& mut self , ct : ty :: Const < 'tcx >) -> Self :: Result { if ct . has_type_flags (ty :: TypeFlags :: HAS_TY_INFER) { ct . super_visit_with (self) } } }
    };
}

impl_198!()