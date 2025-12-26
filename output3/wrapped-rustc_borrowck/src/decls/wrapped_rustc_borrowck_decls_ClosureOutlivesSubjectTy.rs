use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Represents a `ty::Ty` for use in [`ClosureOutlivesSubject`].
///
/// This abstraction is necessary because the type may include `ReVar` regions,
/// which is what we use internally within NLL code, and they can't be used in
/// a query response.
#[derive(Copy, Clone, Debug)]
pub struct ClosureOutlivesSubjectTy<'tcx> {
    inner: Ty<'tcx>,
}
