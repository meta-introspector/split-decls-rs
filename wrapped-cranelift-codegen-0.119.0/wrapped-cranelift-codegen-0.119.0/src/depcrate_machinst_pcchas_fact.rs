// Generated macro for has_fact (function)
macro_rules! Depcrate_machinst_pcchas_fact {
() => {
// Module: crate::machinst::pcc
// Provides: {"has_fact"}
// Dependencies: {}
pub (crate) fn has_fact < I : VCodeInst > (vcode : & VCode < I > , reg : Reg) -> bool { vcode . vreg_fact (reg . into ()) . is_some () }
};
}
