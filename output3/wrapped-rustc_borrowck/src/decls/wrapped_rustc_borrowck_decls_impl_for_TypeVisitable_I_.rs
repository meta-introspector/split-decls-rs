use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<'tcx, I> !TypeVisitable<I> for ClosureOutlivesSubjectTy<'tcx> {}
