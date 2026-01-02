// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_infer/src/infer/canonical/query_response.rs
// Error: expected square brackets
// Problematic line: line 21

use tracing::{debug, instrument};

use crate::infer::canonical::instantiate::{CanonicalExt, instantiate_value};
use crate::infer::canonical::{
    Canonical, CanonicalQueryResponse, CanonicalVarValues, Certainty, OriginalQueryValues,
    QueryRegionConstraints, QueryResponse,
};
