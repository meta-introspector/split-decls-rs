// Generated macro for impl_2053 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2053 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2053"}
// Dependencies: {}
impl FpuOPRRR { pub (crate) fn op_name (self , width : FpuOPWidth) -> String { match self { Self :: Fadd => format ! ("fadd.{width}") , Self :: Fsub => format ! ("fsub.{width}") , Self :: Fmul => format ! ("fmul.{width}") , Self :: Fdiv => format ! ("fdiv.{width}") , Self :: Fsgnj => format ! ("fsgnj.{width}") , Self :: Fsgnjn => format ! ("fsgnjn.{width}") , Self :: Fsgnjx => format ! ("fsgnjx.{width}") , Self :: Fmin => format ! ("fmin.{width}") , Self :: Fmax => format ! ("fmax.{width}") , Self :: Feq => format ! ("feq.{width}") , Self :: Flt => format ! ("flt.{width}") , Self :: Fle => format ! ("fle.{width}") , Self :: Fminm => format ! ("fminm.{width}") , Self :: Fmaxm => format ! ("fmaxm.{width}") , } } pub (crate) fn opcode (self) -> u32 { 0b1010011 } pub (crate) const fn funct5 (self) -> u32 { match self { Self :: Fadd => 0b00000 , Self :: Fsub => 0b00001 , Self :: Fmul => 0b00010 , Self :: Fdiv => 0b00011 , Self :: Fsgnj => 0b00100 , Self :: Fsgnjn => 0b00100 , Self :: Fsgnjx => 0b00100 , Self :: Fmin => 0b00101 , Self :: Fmax => 0b00101 , Self :: Feq => 0b10100 , Self :: Flt => 0b10100 , Self :: Fle => 0b10100 , Self :: Fminm => 0b00101 , Self :: Fmaxm => 0b00101 , } } pub (crate) fn funct7 (self , width : FpuOPWidth) -> u32 { (self . funct5 () << 2) | width . as_u32 () } pub (crate) fn has_frm (self) -> bool { match self { FpuOPRRR :: Fsgnj | FpuOPRRR :: Fsgnjn | FpuOPRRR :: Fsgnjx | FpuOPRRR :: Fmin | FpuOPRRR :: Fmax | FpuOPRRR :: Feq | FpuOPRRR :: Flt | FpuOPRRR :: Fle => false , _ => true , } } }
};
}
