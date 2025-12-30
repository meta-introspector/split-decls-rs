// Generated macro for BorrowckInferCtxt (struct)
macro_rules! DepcrateBorrowckInferCtxt {
() => {
// Module: crate
// Provides: {"BorrowckInferCtxt"}
// Dependencies: {}
pub (crate) struct BorrowckInferCtxt < 'tcx > { pub (crate) infcx : InferCtxt < 'tcx > , pub (crate) root_def_id : LocalDefId , pub (crate) param_env : ParamEnv < 'tcx > , pub (crate) reg_var_to_origin : RefCell < FxIndexMap < ty :: RegionVid , RegionCtxt > > , }
};
}
