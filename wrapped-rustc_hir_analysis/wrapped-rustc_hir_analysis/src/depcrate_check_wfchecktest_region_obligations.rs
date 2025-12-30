// Generated macro for test_region_obligations (function)
macro_rules! Depcrate_check_wfchecktest_region_obligations {
() => {
// Module: crate::check::wfcheck
// Provides: {"test_region_obligations"}
// Dependencies: {}
# [doc = " Given a known `param_env` and a set of well formed types, set up an"] # [doc = " `InferCtxt`, call the passed function (to e.g. set up region constraints"] # [doc = " to be tested), then resolve region and return errors"] fn test_region_obligations < 'tcx > (tcx : TyCtxt < 'tcx > , id : LocalDefId , param_env : ty :: ParamEnv < 'tcx > , wf_tys : & FxIndexSet < Ty < 'tcx > > , add_constraints : impl FnOnce (& InferCtxt < 'tcx >) ,) -> bool { let infcx = tcx . infer_ctxt () . build (TypingMode :: non_body_analysis ()) ; add_constraints (& infcx) ; let errors = infcx . resolve_regions (id , param_env , wf_tys . iter () . copied ()) ; debug ! (? errors , "errors") ; errors . is_empty () }
};
}
