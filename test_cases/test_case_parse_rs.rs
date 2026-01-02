// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_session/src/parse.rs
// Error: expected square brackets
// Problematic line: line 13

use rustc_data_structures::sync::{AppendOnlyVec, Lock};
use rustc_errors::emitter::{FatalOnlyEmitter, HumanEmitter, stderr_destination};
use rustc_errors::translation::Translator;
use rustc_errors::{
    BufferedEarlyLint, ColorConfig, DecorateDiagCompat, Diag, DiagCtxt, DiagCtxtHandle,
    DiagMessage, EmissionGuarantee, MultiSpan, StashKey,
};
