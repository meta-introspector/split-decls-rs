// Generated macro for is_reg_saved_in_prologue (function)
macro_rules! Depcrate_isa_aarch64_abiis_reg_saved_in_prologue {
() => {
// Module: crate::isa::aarch64::abi
// Provides: {"is_reg_saved_in_prologue"}
// Dependencies: {}
# [doc = " Is the given register saved in the prologue if clobbered, i.e., is it a"] # [doc = " callee-save?"] fn is_reg_saved_in_prologue (_call_conv : isa :: CallConv , enable_pinned_reg : bool , sig : & Signature , r : RealReg ,) -> bool { let save_z_regs = sig . params . iter () . filter (| p | p . value_type . is_dynamic_vector ()) . count () != 0 ; match r . class () { RegClass :: Int => { if enable_pinned_reg && r . hw_enc () == PINNED_REG { false } else { r . hw_enc () >= 19 && r . hw_enc () <= 28 } } RegClass :: Float => { if save_z_regs { r . hw_enc () >= 8 && r . hw_enc () <= 23 } else { r . hw_enc () >= 8 && r . hw_enc () <= 15 } } RegClass :: Vector => unreachable ! () , } }
};
}
