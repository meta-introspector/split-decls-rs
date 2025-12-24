use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// When `check_fn` is invoked on a coroutine (i.e., a body that
/// includes yield), it returns back some information about the yield
/// points.
#[derive(Debug, PartialEq, Copy, Clone)]
struct CoroutineTypes<'tcx> {
    /// Type of coroutine argument / values returned by `yield`.
    resume_ty: Ty<'tcx>,
    /// Type of value that is yielded.
    yield_ty: Ty<'tcx>,
}
