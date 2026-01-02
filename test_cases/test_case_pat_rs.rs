// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_pattern_analysis/src/pat.rs
// Error: expected square brackets
// Problematic line: line 12

use crate::constructor::{Constructor, Slice, SliceKind};
use crate::{PatCx, PrivateUninhabitedField};

/// A globally unique id to distinguish patterns.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct PatId(u32);
impl PatId {
