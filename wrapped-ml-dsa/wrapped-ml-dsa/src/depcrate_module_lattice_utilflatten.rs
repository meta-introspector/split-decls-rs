// Generated macro for Flatten (trait)
macro_rules! Depcrate_module_lattice_utilFlatten {
() => {
// Module: crate::module_lattice::util
// Provides: {"Flatten"}
// Dependencies: {}
# [doc = " Defines a sequence of sequences that can be merged into a bigger overall seequence"] pub (crate) trait Flatten < T , M : ArraySize > { type OutputSize : ArraySize ; fn flatten (self) -> Array < T , Self :: OutputSize > ; }
};
}
