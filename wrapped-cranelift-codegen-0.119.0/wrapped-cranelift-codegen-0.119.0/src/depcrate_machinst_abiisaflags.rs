// Generated macro for IsaFlags (trait)
macro_rules! Depcrate_machinst_abiIsaFlags {
() => {
// Module: crate::machinst::abi
// Provides: {"IsaFlags"}
// Dependencies: {}
# [doc = " Trait implemented by machine-specific backend to represent ISA flags."] pub trait IsaFlags : Clone { # [doc = " Get a flag indicating whether forward-edge CFI is enabled."] fn is_forward_edge_cfi_enabled (& self) -> bool { false } }
};
}
