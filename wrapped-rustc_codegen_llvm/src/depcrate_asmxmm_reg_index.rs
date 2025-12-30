// Generated macro for xmm_reg_index (function)
macro_rules! Depcrate_asmxmm_reg_index {
() => {
// Module: crate::asm
// Provides: {"xmm_reg_index"}
// Dependencies: {}
# [doc = " If the register is an xmm/ymm/zmm register then return its index."] fn xmm_reg_index (reg : InlineAsmReg) -> Option < u32 > { use X86InlineAsmReg :: * ; match reg { InlineAsmReg :: X86 (reg) if reg as u32 >= xmm0 as u32 && reg as u32 <= xmm15 as u32 => { Some (reg as u32 - xmm0 as u32) } InlineAsmReg :: X86 (reg) if reg as u32 >= ymm0 as u32 && reg as u32 <= ymm15 as u32 => { Some (reg as u32 - ymm0 as u32) } InlineAsmReg :: X86 (reg) if reg as u32 >= zmm0 as u32 && reg as u32 <= zmm31 as u32 => { Some (reg as u32 - zmm0 as u32) } _ => None , } }
};
}
