// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/traits/query.rs
// Error: expected square brackets
// Problematic line: line 16

pub use crate::traits::solve::NoSolution;
use crate::ty::{self, GenericArg, Ty, TyCtxt};

pub mod type_op {
    use rustc_macros::{HashStable, TypeFoldable, TypeVisitable};

    use crate::ty::{Predicate, Ty, UserType};
