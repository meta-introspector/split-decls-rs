// Generated macro for CmpOpcode (enum)
macro_rules! Depcrate_isa_x64_inst_argsCmpOpcode {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"CmpOpcode"}
// Dependencies: {}
# [derive (Clone , Copy , PartialEq)] # [doc = " Comparison operations."] pub enum CmpOpcode { # [doc = " CMP instruction: compute `a - b` and set flags from result."] Cmp , # [doc = " TEST instruction: compute `a & b` and set flags from result."] Test , }
};
}
