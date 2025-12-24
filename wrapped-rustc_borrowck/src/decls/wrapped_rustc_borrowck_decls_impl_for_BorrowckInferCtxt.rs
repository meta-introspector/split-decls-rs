use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'tcx> BorrowckInferCtxt<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>, def_id: LocalDefId, root_def_id: LocalDefId) -> Self {
        let typing_mode = if tcx.use_typing_mode_borrowck() {
            TypingMode::borrowck(tcx, def_id)
        } else {
            TypingMode::analysis_in_body(tcx, def_id)
        };
        let infcx = tcx.infer_ctxt().build(typing_mode);
        let param_env = tcx.param_env(def_id);
        BorrowckInferCtxt {
            infcx,
            root_def_id,
            reg_var_to_origin: RefCell::new(Default::default()),
            param_env,
        }
    }
    pub fn next_region_var<F>(
        &self,
        origin: RegionVariableOrigin,
        get_ctxt_fn: F,
    ) -> ty::Region<'tcx>
    where
        F: Fn() -> RegionCtxt,
    {
        let next_region = self.infcx.next_region_var(origin);
        let vid = next_region.as_var();
        if cfg!(debug_assertions) {
            debug!(
                "inserting vid {:?} with origin {:?} into var_to_origin", vid, origin
            );
            let ctxt = get_ctxt_fn();
            let mut var_to_origin = self.reg_var_to_origin.borrow_mut();
            assert_eq!(var_to_origin.insert(vid, ctxt), None);
        }
        next_region
    }
    #[instrument(skip(self, get_ctxt_fn), level = "debug")]
    pub fn next_nll_region_var<F>(
        &self,
        origin: NllRegionVariableOrigin,
        get_ctxt_fn: F,
    ) -> ty::Region<'tcx>
    where
        F: Fn() -> RegionCtxt,
    {
        let next_region = self.infcx.next_nll_region_var(origin);
        let vid = next_region.as_var();
        if cfg!(debug_assertions) {
            debug!(
                "inserting vid {:?} with origin {:?} into var_to_origin", vid, origin
            );
            let ctxt = get_ctxt_fn();
            let mut var_to_origin = self.reg_var_to_origin.borrow_mut();
            assert_eq!(var_to_origin.insert(vid, ctxt), None);
        }
        next_region
    }
}
