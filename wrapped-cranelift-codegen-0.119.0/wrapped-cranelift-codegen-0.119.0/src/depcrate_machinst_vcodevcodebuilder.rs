// Generated macro for VCodeBuilder (struct)
macro_rules! Depcrate_machinst_vcodeVCodeBuilder {
() => {
// Module: crate::machinst::vcode
// Provides: {"VCodeBuilder"}
// Dependencies: {}
# [doc = " A builder for a VCode function body."] # [doc = ""] # [doc = " This builder has the ability to accept instructions in either"] # [doc = " forward or reverse order, depending on the pass direction that"] # [doc = " produces the VCode. The lowering from CLIF to VCode<MachInst>"] # [doc = " ordinarily occurs in reverse order (in order to allow instructions"] # [doc = " to be lowered only if used, and not merged) so a reversal will"] # [doc = " occur at the end of lowering to ensure the VCode is in machine"] # [doc = " order."] # [doc = ""] # [doc = " If built in reverse, block and instruction indices used once the"] # [doc = " VCode is built are relative to the final (reversed) order, not the"] # [doc = " order of construction. Note that this means we do not know the"] # [doc = " final block or instruction indices when building, so we do not"] # [doc = " hand them out. (The user is assumed to know them when appending"] # [doc = " terminator instructions with successor blocks.)"] pub struct VCodeBuilder < I : VCodeInst > { # [doc = " In-progress VCode."] pub (crate) vcode : VCode < I > , # [doc = " In what direction is the build occurring?"] direction : VCodeBuildDirection , # [doc = " Debug-value label in-progress map, keyed by label. For each"] # [doc = " label, we keep disjoint ranges mapping to vregs. We'll flatten"] # [doc = " this into (vreg, range, label) tuples when done."] debug_info : FxHashMap < ValueLabel , Vec < (InsnIndex , InsnIndex , VReg) > > , }
};
}
