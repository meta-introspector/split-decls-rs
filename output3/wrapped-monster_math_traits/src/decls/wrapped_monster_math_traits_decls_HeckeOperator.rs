use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A trait representing a Hecke operator for applying algebraic transformations to declarations.
pub trait HeckeOperator {
    /// Applies an algebraic transformation (Hecke operator) to a given Declaration.
    /// Returns a new, transformed Declaration.
    fn apply_transformation(&self, declaration: Declaration) -> Declaration;
}
