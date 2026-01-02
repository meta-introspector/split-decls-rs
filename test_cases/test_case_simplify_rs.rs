// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/simplify.rs
// Error: expected square brackets
// Problematic line: line 46

use smallvec::SmallVec;
use tracing::{debug, trace};

pub(super) enum SimplifyCfg {
    Initial,
    PromoteConsts,
    RemoveFalseEdges,
