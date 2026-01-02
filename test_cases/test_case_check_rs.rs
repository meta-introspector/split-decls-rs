// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_const_eval/src/check_consts/check.rs
// Error: expected square brackets
// Problematic line: line 23

use rustc_mir_dataflow::Analysis;
use rustc_mir_dataflow::impls::{MaybeStorageLive, always_storage_live_locals};
use rustc_span::{Span, Symbol, sym};
use rustc_trait_selection::traits::{
    Obligation, ObligationCause, ObligationCauseCode, ObligationCtxt,
};
use tracing::{instrument, trace};
