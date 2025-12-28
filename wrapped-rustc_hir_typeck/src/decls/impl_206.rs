macro_rules! deps {
    () => {
        NestedObligationsForSelfTy!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl < 'a , 'tcx > ProofTreeVisitor < 'tcx > for NestedObligationsForSelfTy < 'a , 'tcx > { fn span (& self) -> Span { self . root_cause . span } fn config (& self) -> InspectConfig { InspectConfig { max_depth : 5 } } fn visit_goal (& mut self , inspect_goal : & InspectGoal < '_ , 'tcx >) { if inspect_goal . result () == Ok (Certainty :: Yes) { return ; } let tcx = self . fcx . tcx ; let goal = inspect_goal . goal () ; if self . fcx . predicate_has_self_ty (goal . predicate , self . self_ty) && ! matches ! (inspect_goal . source () , GoalSource :: InstantiateHigherRanked) { self . obligations_for_self_ty . push (traits :: Obligation :: new (tcx , self . root_cause . clone () , goal . param_env , goal . predicate ,)) ; } if let Some (candidate) = inspect_goal . unique_applicable_candidate () { candidate . visit_nested_no_probe (self) } } }
    };
}

impl_206!();