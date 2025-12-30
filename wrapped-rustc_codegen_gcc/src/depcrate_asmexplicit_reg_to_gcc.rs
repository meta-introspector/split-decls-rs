// Generated macro for explicit_reg_to_gcc (function)
macro_rules! Depcrate_asmexplicit_reg_to_gcc {
() => {
// Module: crate::asm
// Provides: {"explicit_reg_to_gcc"}
// Dependencies: {}
fn explicit_reg_to_gcc (reg : InlineAsmReg) -> & 'static str { match reg { InlineAsmReg :: X86 (reg) => { match reg . reg_class () { X86InlineAsmRegClass :: reg_byte => { reg . name () . trim_end_matches ('b') } _ => match reg . name () { "st(0)" => "st" , name => name , } , } } InlineAsmReg :: Arm (reg) => reg . name () , InlineAsmReg :: AArch64 (reg) => reg . name () , _ => unimplemented ! () , } }
};
}
