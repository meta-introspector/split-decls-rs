// Generated macro for get_fact_or_default (function)
macro_rules! Depcrate_machinst_pccget_fact_or_default {
() => {
// Module: crate::machinst::pcc
// Provides: {"get_fact_or_default"}
// Dependencies: {}
pub (crate) fn get_fact_or_default < I : VCodeInst > (vcode : & VCode < I > , reg : Reg , width : u16) -> Fact { trace ! ("get_fact_or_default: reg {reg:?} -> {:?}" , vcode . vreg_fact (reg . into ())) ; vcode . vreg_fact (reg . into ()) . cloned () . unwrap_or_else (| | Fact :: max_range_for_width (width)) }
};
}
