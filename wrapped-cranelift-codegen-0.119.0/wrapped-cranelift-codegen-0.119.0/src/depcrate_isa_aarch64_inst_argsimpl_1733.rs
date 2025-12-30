// Generated macro for impl_1733 (impl)
macro_rules! Depcrate_isa_aarch64_inst_argsimpl_1733 {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"impl_1733"}
// Dependencies: {}
impl AMode { # [doc = " Memory reference using an address in a register."] pub fn reg (reg : Reg) -> AMode { AMode :: UnsignedOffset { rn : reg , uimm12 : UImm12Scaled :: zero (I64) , } } # [doc = " Memory reference using `reg1 + sizeof(ty) * reg2` as an address, with `reg2` sign- or"] # [doc = " zero-extended as per `op`."] pub fn reg_plus_reg_scaled_extended (reg1 : Reg , reg2 : Reg , op : ExtendOp) -> AMode { AMode :: RegScaledExtended { rn : reg1 , rm : reg2 , extendop : op , } } }
};
}
