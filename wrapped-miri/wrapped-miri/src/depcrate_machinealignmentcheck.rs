// Generated macro for AlignmentCheck (enum)
macro_rules! Depcrate_machineAlignmentCheck {
() => {
// Module: crate::machine
// Provides: {"AlignmentCheck"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , PartialEq)] pub enum AlignmentCheck { # [doc = " Do not check alignment."] None , # [doc = " Check alignment \"symbolically\", i.e., using only the requested alignment for an allocation and not its real base address."] Symbolic , # [doc = " Check alignment on the actual physical integer address."] Int , }
};
}
