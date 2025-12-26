use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// This is implemented per query. It allows restoring query values from their erased state
/// and constructing a QueryConfig.
trait QueryConfigRestored<'tcx> {
    type RestoredValue;
    type Config: QueryConfig<QueryCtxt<'tcx>>;
    const NAME: &'static &'static str;
    fn config(tcx: TyCtxt<'tcx>) -> Self::Config;
    fn restore(value: <Self::Config as QueryConfig<QueryCtxt<'tcx>>>::Value)
        -> Self::RestoredValue;
}
