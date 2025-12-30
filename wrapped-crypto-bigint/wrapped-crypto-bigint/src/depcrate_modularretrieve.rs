// Generated macro for Retrieve (trait)
macro_rules! Depcrate_modularRetrieve {
() => {
// Module: crate::modular
// Provides: {"Retrieve"}
// Dependencies: {}
# [doc = " A generalization for numbers kept in optimized representations (e.g. Montgomery)"] # [doc = " that can be converted back to the original form."] pub trait Retrieve { # [doc = " The original type."] type Output ; # [doc = " Convert the number back from the optimized representation."] fn retrieve (& self) -> Self :: Output ; }
};
}
