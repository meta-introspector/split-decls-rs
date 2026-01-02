// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_type_ir/src/search_graph/mod.rs
// Error: expected square brackets
// Problematic line: line 23

use std::marker::PhantomData;

use derive_where::derive_where;
#[cfg(feature = "nightly")]
use rustc_macros::{Decodable_NoContext, Encodable_NoContext, HashStable_NoContext};
use rustc_type_ir::data_structures::HashMap;
use tracing::{debug, instrument};
