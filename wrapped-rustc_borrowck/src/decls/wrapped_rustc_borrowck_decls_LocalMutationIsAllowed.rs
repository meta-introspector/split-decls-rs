use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// When checking permissions for a place access, this flag is used to indicate that an immutable
/// local place can be mutated.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum LocalMutationIsAllowed {
    Yes,
    /// We want use of immutable upvars to cause a "write to immutable upvar"
    /// error, not an "reassignment" error.
    ExceptUpvars,
    No,
}
