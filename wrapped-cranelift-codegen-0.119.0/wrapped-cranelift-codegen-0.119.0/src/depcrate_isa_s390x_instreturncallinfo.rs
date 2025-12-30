// Generated macro for ReturnCallInfo (struct)
macro_rules! Depcrate_isa_s390x_instReturnCallInfo {
() => {
// Module: crate::isa::s390x::inst
// Provides: {"ReturnCallInfo"}
// Dependencies: {}
# [doc = " Additional information for (direct) ReturnCall instructions, left out of line to lower the size of"] # [doc = " the Inst enum."] # [derive (Clone , Debug)] pub struct ReturnCallInfo < T > { pub dest : T , pub uses : CallArgList , pub callee_pop_size : u32 , }
};
}
