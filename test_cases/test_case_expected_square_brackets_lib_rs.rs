// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_span/src/lib.rs
// Error: expected square brackets
// Error type: expected_square_brackets
// Sample #1 of 3
// Problematic line: line 57

use edition::Edition;
pub mod hygiene;
use hygiene::Transparency;
pub use hygiene::{
    DesugaringKind, ExpnData, ExpnHash, ExpnId, ExpnKind, LocalExpnId, MacroKind, SyntaxContext,
};
use rustc_data_structures::stable_hasher::HashingControls;
