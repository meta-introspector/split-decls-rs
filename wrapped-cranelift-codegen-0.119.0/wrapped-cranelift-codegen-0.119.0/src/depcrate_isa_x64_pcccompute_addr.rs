// Generated macro for compute_addr (function)
macro_rules! Depcrate_isa_x64_pcccompute_addr {
() => {
// Module: crate::isa::x64::pcc
// Provides: {"compute_addr"}
// Dependencies: {}
fn compute_addr (ctx : & FactContext , vcode : & VCode < Inst > , amode : & Amode , bits : u16) -> Option < Fact > { trace ! ("compute_addr: {:?}" , amode) ; match * amode { Amode :: ImmReg { simm32 , base , .. } => { let base = get_fact_or_default (vcode , base , bits) ; trace ! ("base = {:?}" , base) ; let simm32 : i64 = simm32 . into () ; let simm32 : u64 = simm32 as u64 ; let offset = Fact :: constant (bits , simm32) ; let sum = ctx . add (& base , & offset , bits) ? ; trace ! ("sum = {:?}" , sum) ; Some (sum) } Amode :: ImmRegRegShift { simm32 , base , index , shift , .. } => { let base = get_fact_or_default (vcode , base . into () , bits) ; let index = get_fact_or_default (vcode , index . into () , bits) ; trace ! ("base = {:?} index = {:?}" , base , index) ; let shifted = ctx . shl (& index , bits , shift . into ()) ? ; let sum = ctx . add (& base , & shifted , bits) ? ; let simm32 : i64 = simm32 . into () ; let simm32 : u64 = simm32 as u64 ; let offset = Fact :: constant (bits , simm32) ; let sum = ctx . add (& sum , & offset , bits) ? ; trace ! ("sum = {:?}" , sum) ; Some (sum) } Amode :: RipRelative { .. } => None , } }
};
}
