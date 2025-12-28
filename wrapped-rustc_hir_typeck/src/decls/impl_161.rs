macro_rules! deps {
    () => {
        Error!();
        TypeInformationCtxt!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < 'tcx > TypeInformationCtxt < 'tcx > for (& LateContext < 'tcx > , LocalDefId) { type TypeckResults < 'a > = & 'tcx ty :: TypeckResults < 'tcx > where Self : 'a ; type Error = ! ; fn typeck_results (& self) -> Self :: TypeckResults < '_ > { self . 0 . maybe_typeck_results () . expect ("expected typeck results") } fn structurally_resolve_type (& self , _span : Span , ty : Ty < 'tcx >) -> Ty < 'tcx > { ty } fn resolve_vars_if_possible < T : TypeFoldable < TyCtxt < 'tcx > > > (& self , t : T) -> T { t } fn report_bug (& self , span : Span , msg : impl ToString) -> ! { span_bug ! (span , "{}" , msg . to_string ()) } fn error_reported_in_ty (& self , _ty : Ty < 'tcx >) -> Result < () , ! > { Ok (()) } fn tainted_by_errors (& self) -> Result < () , ! > { Ok (()) } fn type_is_copy_modulo_regions (& self , ty : Ty < 'tcx >) -> bool { self . 0 . type_is_copy_modulo_regions (ty) } fn type_is_use_cloned_modulo_regions (& self , ty : Ty < 'tcx >) -> bool { self . 0 . type_is_use_cloned_modulo_regions (ty) } fn body_owner_def_id (& self) -> LocalDefId { self . 1 } fn tcx (& self) -> TyCtxt < 'tcx > { self . 0 . tcx } }
    };
}

impl_161!()