// Generated macro for region_known_to_outlive (function)
macro_rules! Depcrate_check_wfcheckregion_known_to_outlive {
() => {
// Module: crate::check::wfcheck
// Provides: {"region_known_to_outlive"}
// Dependencies: {}
# [doc = " Given a known `param_env` and a set of well formed types, can we prove that"] # [doc = " `region_a` outlives `region_b`"] fn region_known_to_outlive < 'tcx > (tcx : TyCtxt < 'tcx > , id : LocalDefId , param_env : ty :: ParamEnv < 'tcx > , wf_tys : & FxIndexSet < Ty < 'tcx > > , region_a : ty :: Region < 'tcx > , region_b : ty :: Region < 'tcx > ,) -> bool { test_region_obligations (tcx , id , param_env , wf_tys , | infcx | { infcx . sub_regions (SubregionOrigin :: RelateRegionParamBound (DUMMY_SP , None) , region_b , region_a ,) ; }) }
};
}
