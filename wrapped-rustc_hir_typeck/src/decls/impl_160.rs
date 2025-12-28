macro_rules! deps {
    () => {
        Error!();
        TypeInformationCtxt!();
        FnCtxt!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < 'tcx > TypeInformationCtxt < 'tcx > for & FnCtxt < '_ , 'tcx > { type TypeckResults < 'a > = Ref < 'a , ty :: TypeckResults < 'tcx > > where Self : 'a ; type Error = ErrorGuaranteed ; fn typeck_results (& self) -> Self :: TypeckResults < '_ > { self . typeck_results . borrow () } fn resolve_vars_if_possible < T : TypeFoldable < TyCtxt < 'tcx > > > (& self , t : T) -> T { self . infcx . resolve_vars_if_possible (t) } fn structurally_resolve_type (& self , sp : Span , ty : Ty < 'tcx >) -> Ty < 'tcx > { (* * self) . structurally_resolve_type (sp , ty) } fn report_bug (& self , span : Span , msg : impl ToString) -> Self :: Error { self . dcx () . span_delayed_bug (span , msg . to_string ()) } fn error_reported_in_ty (& self , ty : Ty < 'tcx >) -> Result < () , Self :: Error > { ty . error_reported () } fn tainted_by_errors (& self) -> Result < () , ErrorGuaranteed > { if let Some (guar) = self . infcx . tainted_by_errors () { Err (guar) } else { Ok (()) } } fn type_is_copy_modulo_regions (& self , ty : Ty < 'tcx >) -> bool { self . infcx . type_is_copy_modulo_regions (self . param_env , ty) } fn type_is_use_cloned_modulo_regions (& self , ty : Ty < 'tcx >) -> bool { self . infcx . type_is_use_cloned_modulo_regions (self . param_env , ty) } fn body_owner_def_id (& self) -> LocalDefId { self . body_id } fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } }
    };
}

impl_160!();