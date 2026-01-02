// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_transform/src/instsimplify.rs
// Error: expected square brackets
// Error type: expected_square_brackets
// Sample #2 of 3
// Problematic line: line 14


use crate::simplify::simplify_duplicate_switch_targets;

pub(super) enum InstSimplify {
    BeforeInline,
    AfterSimplifyCfg,
}
