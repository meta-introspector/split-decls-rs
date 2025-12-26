use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EagerCallInfo {
    /// The expanded argument of the eager macro.
    arg: Arc<tt::TopSubtree>,
    /// Call id of the eager macro's input file (this is the macro file for its fully expanded input).
    arg_id: MacroCallId,
    error: Option<ExpandError>,
    /// The call site span of the eager macro
    span: Span,
}
