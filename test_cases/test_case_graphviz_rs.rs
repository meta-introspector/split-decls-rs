// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_borrowck/src/region_infer/graphviz.rs
// Error: expected square brackets
// Problematic line: line 14


use super::*;

fn render_outlives_constraint(constraint: &OutlivesConstraint<'_>) -> String {
    if let ConstraintCategory::OutlivesUnnameablePlaceholder(unnameable) = constraint.category {
        format!("{unnameable:?} unnameable")
    } else {
