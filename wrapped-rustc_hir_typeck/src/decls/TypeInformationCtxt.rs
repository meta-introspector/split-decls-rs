macro_rules! deps {
    () => {
        ExprUseVisitor!();
        FnCtxt!();
        Error!();
    };
}

macro_rules! TypeInformationCtxt {
    () => {
        deps!();
        # [doc = " This trait makes `ExprUseVisitor` usable with both [`FnCtxt`]"] # [doc = " and [`LateContext`], depending on where in the compiler it is used."] pub trait TypeInformationCtxt < 'tcx > { type TypeckResults < 'a > : Deref < Target = ty :: TypeckResults < 'tcx > > where Self : 'a ; type Error ; fn typeck_results (& self) -> Self :: TypeckResults < '_ > ; fn resolve_vars_if_possible < T : TypeFoldable < TyCtxt < 'tcx > > > (& self , t : T) -> T ; fn structurally_resolve_type (& self , span : Span , ty : Ty < 'tcx >) -> Ty < 'tcx > ; fn report_bug (& self , span : Span , msg : impl ToString) -> Self :: Error ; fn error_reported_in_ty (& self , ty : Ty < 'tcx >) -> Result < () , Self :: Error > ; fn tainted_by_errors (& self) -> Result < () , Self :: Error > ; fn type_is_copy_modulo_regions (& self , ty : Ty < 'tcx >) -> bool ; fn type_is_use_cloned_modulo_regions (& self , ty : Ty < 'tcx >) -> bool ; fn body_owner_def_id (& self) -> LocalDefId ; fn tcx (& self) -> TyCtxt < 'tcx > ; }
    };
}

TypeInformationCtxt!();