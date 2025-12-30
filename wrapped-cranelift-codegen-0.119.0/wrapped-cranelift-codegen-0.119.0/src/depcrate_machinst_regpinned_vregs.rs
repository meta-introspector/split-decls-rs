// Generated macro for PINNED_VREGS (const)
macro_rules! Depcrate_machinst_regPINNED_VREGS {
() => {
// Module: crate::machinst::reg
// Provides: {"PINNED_VREGS"}
// Dependencies: {}
# [doc = " The first 192 vregs (64 int, 64 float, 64 vec) are \"pinned\" to"] # [doc = " physical registers: this means that they are always constrained to"] # [doc = " the corresponding register at all use/mod/def sites."] # [doc = ""] # [doc = " Arbitrary vregs can also be constrained to physical registers at"] # [doc = " particular use/def/mod sites, and this is preferable; but pinned"] # [doc = " vregs allow us to migrate code that has been written using"] # [doc = " RealRegs directly."] const PINNED_VREGS : usize = 192 ;
};
}
