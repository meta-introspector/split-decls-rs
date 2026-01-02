// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_query_system/src/dep_graph/debug.rs
// Error: expected square brackets
// Problematic line: line 10


use super::{DepNode, DepNodeIndex};

/// A dep-node filter goes from a user-defined string to a query over
/// nodes. Right now the format is like this:
/// ```ignore (illustrative)
/// x & y & z
