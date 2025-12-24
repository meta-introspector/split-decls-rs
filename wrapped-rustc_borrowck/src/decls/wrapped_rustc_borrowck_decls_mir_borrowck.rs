use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Provider for `query mir_borrowck`. Similar to `typeck`, this must
/// only be called for typeck roots which will then borrowck all
/// nested bodies as well.
fn mir_borrowck(
    tcx: TyCtxt<'_>,
    def: LocalDefId,
) -> Result<&FxIndexMap<LocalDefId, ty::DefinitionSiteHiddenType<'_>>, ErrorGuaranteed> {
    assert!(! tcx.is_typeck_child(def.to_def_id()));
    let (input_body, _) = tcx.mir_promoted(def);
    debug!("run query mir_borrowck: {}", tcx.def_path_str(def));
    let input_body: &Body<'_> = &input_body.borrow();
    if let Some(guar) = input_body.tainted_by_errors {
        debug!("Skipping borrowck because of tainted body");
        Err(guar)
    } else if input_body.should_skip() {
        debug!("Skipping borrowck because of injected body");
        let opaque_types = Default::default();
        Ok(tcx.arena.alloc(opaque_types))
    } else {
        let mut root_cx = BorrowCheckRootCtxt::new(tcx, def, None);
        root_cx.do_mir_borrowck();
        root_cx.finalize()
    }
}
