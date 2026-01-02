// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_feature/src/accepted.rs
// Error: expected square brackets
// Problematic line: line 7


use super::{Feature, to_nonzero};

macro_rules! declare_features {
    ($(
        $(#[doc = $doc:tt])* (accepted, $feature:ident, $ver:expr, $issue:expr),
    )+) => {
