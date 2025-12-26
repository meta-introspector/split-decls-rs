use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl MockPredicateObligation {
    pub fn new(
        _tcx: TyCtxt,
        _cause: MockObligationCause,
        _param_env: MockParamEnv,
        _predicate: MockPredicate,
    ) -> Self {
        MockPredicateObligation
    }
}
