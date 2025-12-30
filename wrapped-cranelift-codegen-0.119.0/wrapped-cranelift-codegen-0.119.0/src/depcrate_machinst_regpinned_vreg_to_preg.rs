// Generated macro for pinned_vreg_to_preg (function)
macro_rules! Depcrate_machinst_regpinned_vreg_to_preg {
() => {
// Module: crate::machinst::reg
// Provides: {"pinned_vreg_to_preg"}
// Dependencies: {}
# [doc = " Convert a `VReg` to its pinned `PReg`, if any."] pub fn pinned_vreg_to_preg (vreg : VReg) -> Option < PReg > { if vreg . vreg () < PINNED_VREGS { Some (PReg :: from_index (vreg . vreg ())) } else { None } }
};
}
