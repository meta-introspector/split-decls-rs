// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_public/src/crate_def.rs
// Error: expected square brackets
// Problematic line: line 9

use crate::ty::{GenericArgs, Span, Ty};
use crate::{AssocItems, Crate, Symbol, with};

/// A unique identification number for each item accessible for the current compilation unit.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct DefId(pub(crate) usize);

