// Generated macro for is_reg_saved_in_prologue (function)
macro_rules! Depcrate_isa_s390x_abiis_reg_saved_in_prologue {
() => {
// Module: crate::isa::s390x::abi
// Provides: {"is_reg_saved_in_prologue"}
// Dependencies: {}
fn is_reg_saved_in_prologue (call_conv : isa :: CallConv , r : RealReg) -> bool { match (call_conv , r . class ()) { (isa :: CallConv :: Tail , RegClass :: Int) => { r . hw_enc () >= 8 && r . hw_enc () <= 15 } (_ , RegClass :: Int) => { r . hw_enc () >= 6 && r . hw_enc () <= 15 } (_ , RegClass :: Float) => { r . hw_enc () >= 8 && r . hw_enc () <= 15 } (_ , RegClass :: Vector) => unreachable ! () , } }
};
}
