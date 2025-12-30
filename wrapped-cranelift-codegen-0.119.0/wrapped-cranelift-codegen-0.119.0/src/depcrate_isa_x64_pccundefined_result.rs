// Generated macro for undefined_result (function)
macro_rules! Depcrate_isa_x64_pccundefined_result {
() => {
// Module: crate::isa::x64::pcc
// Provides: {"undefined_result"}
// Dependencies: {}
fn undefined_result (ctx : & FactContext , vcode : & mut VCode < Inst > , dst : Writable < Gpr > , reg_bits : u16 , result_bits : u16 ,) -> PccResult < () > { check_output (ctx , vcode , dst . to_writable_reg () , & [] , | _vcode | { clamp_range (ctx , reg_bits , result_bits , None) }) }
};
}
