use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A trait for checking the conformity of a Declaration to Monster Group properties.
pub trait MonsterConformityChecker {
    /// Checks if a Declaration conforms to the specified Monster Group properties.
    /// It can optionally use BottPeriodicityTrait for enhanced soundness checks.
    fn check_conformity(
        &self,
        declaration: &Declaration,
        bott_periodicity_checker: Option<&dyn BottPeriodicityTrait>,
        constants: &dyn MonsterConstants,
    ) -> bool;
}
