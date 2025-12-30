// Generated macro for impl_1306 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1306 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1306"}
// Dependencies: {}
impl RegMemImm { # [doc = " Create a register operand."] pub fn reg (reg : Reg) -> Self { debug_assert ! (reg . class () == RegClass :: Int || reg . class () == RegClass :: Float) ; Self :: Reg { reg } } # [doc = " Create a memory operand."] pub fn mem (addr : impl Into < SyntheticAmode >) -> Self { Self :: Mem { addr : addr . into () } } # [doc = " Create an immediate operand."] pub fn imm (simm32 : u32) -> Self { Self :: Imm { simm32 } } # [doc = " Asserts that in register mode, the reg class is the one that's expected."] pub (crate) fn assert_regclass_is (& self , expected_reg_class : RegClass) { if let Self :: Reg { reg } = self { debug_assert_eq ! (reg . class () , expected_reg_class) ; } } # [doc = " Add the regs mentioned by `self` to `collector`."] pub (crate) fn get_operands (& mut self , collector : & mut impl OperandVisitor) { match self { Self :: Reg { reg } => collector . reg_use (reg) , Self :: Mem { addr } => addr . get_operands (collector) , Self :: Imm { .. } => { } } } }
};
}
