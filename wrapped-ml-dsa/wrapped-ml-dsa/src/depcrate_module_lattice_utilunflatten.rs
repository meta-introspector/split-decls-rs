// Generated macro for Unflatten (trait)
macro_rules! Depcrate_module_lattice_utilUnflatten {
() => {
// Module: crate::module_lattice::util
// Provides: {"Unflatten"}
// Dependencies: {}
# [doc = " Defines a sequence that can be split into a sequence of smaller sequences of uniform size"] pub (crate) trait Unflatten < M > where M : ArraySize , { type Part ; fn unflatten (self) -> Array < Self :: Part , M > ; }
};
}
