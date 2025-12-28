macro_rules! ty_known_to_outlive {
    () => {
        # [doc = " Given a known `param_env` and a set of well formed types, can we prove that"] # [doc = " `ty` outlives `region`."] fn ty_known_to_outlive < 'tcx > (tcx : TyCtxt < 'tcx > , id : LocalDefId , param_env : ty :: ParamEnv < 'tcx > , wf_tys : & FxIndexSet < Ty < 'tcx > > , ty : Ty < 'tcx > , region : ty :: Region < 'tcx > ,) -> bool { test_region_obligations (tcx , id , param_env , wf_tys , | infcx | { infcx . register_type_outlives_constraint_inner (infer :: TypeOutlivesConstraint { sub_region : region , sup_type : ty , origin : SubregionOrigin :: RelateParamBound (DUMMY_SP , ty , None) , }) ; }) }
    };
}

ty_known_to_outlive!()