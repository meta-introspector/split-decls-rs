// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_incremental/src/persist/data.rs
// Error: expected square brackets
// Problematic line: line 6

use rustc_macros::{Decodable, Encodable};
use rustc_middle::dep_graph::{WorkProduct, WorkProductId};

#[derive(Debug, Encodable, Decodable)]
pub(crate) struct SerializedWorkProduct {
    /// node that produced the work-product
    pub id: WorkProductId,
