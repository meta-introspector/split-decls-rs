// Generated macro for realreg_name (function)
macro_rules! Depcrate_isa_x64_inst_regsrealreg_name {
() => {
// Module: crate::isa::x64::inst::regs
// Provides: {"realreg_name"}
// Dependencies: {}
# [doc = " Give the name of a RealReg."] pub fn realreg_name (reg : RealReg) -> & 'static str { let preg = PReg :: from (reg) ; match preg . class () { RegClass :: Int => match preg . hw_enc () as u8 { ENC_RAX => "%rax" , ENC_RBX => "%rbx" , ENC_RCX => "%rcx" , ENC_RDX => "%rdx" , ENC_RSI => "%rsi" , ENC_RDI => "%rdi" , ENC_RBP => "%rbp" , ENC_RSP => "%rsp" , ENC_R8 => "%r8" , ENC_R9 => "%r9" , ENC_R10 => "%r10" , ENC_R11 => "%r11" , ENC_R12 => "%r12" , ENC_R13 => "%r13" , ENC_R14 => "%r14" , ENC_R15 => "%r15" , _ => panic ! ("Invalid PReg: {preg:?}") , } , RegClass :: Float => match preg . hw_enc () { 0 => "%xmm0" , 1 => "%xmm1" , 2 => "%xmm2" , 3 => "%xmm3" , 4 => "%xmm4" , 5 => "%xmm5" , 6 => "%xmm6" , 7 => "%xmm7" , 8 => "%xmm8" , 9 => "%xmm9" , 10 => "%xmm10" , 11 => "%xmm11" , 12 => "%xmm12" , 13 => "%xmm13" , 14 => "%xmm14" , 15 => "%xmm15" , _ => panic ! ("Invalid PReg: {preg:?}") , } , RegClass :: Vector => unreachable ! () , } }
};
}
