use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn generic_arg_from_param(db: &dyn HirDatabase, id: TypeOrConstParamId) -> Option<GenericArg<'_>> {
    let local_idx = hir_ty::param_idx(db, id)?;
    let defaults = db.generic_defaults(id.parent);
    let ty = defaults.get(local_idx)?;
    Some(ty.instantiate_identity())
}
