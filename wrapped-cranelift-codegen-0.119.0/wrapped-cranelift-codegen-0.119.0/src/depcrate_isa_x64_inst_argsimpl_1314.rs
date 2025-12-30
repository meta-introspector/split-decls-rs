// Generated macro for impl_1314 (impl)
macro_rules! Depcrate_isa_x64_inst_argsimpl_1314 {
() => {
// Module: crate::isa::x64::inst::args
// Provides: {"impl_1314"}
// Dependencies: {}
impl RegMem { # [doc = " Create a register operand."] pub fn reg (reg : Reg) -> Self { debug_assert ! (reg . class () == RegClass :: Int || reg . class () == RegClass :: Float) ; Self :: Reg { reg } } # [doc = " Create a memory operand."] pub fn mem (addr : impl Into < SyntheticAmode >) -> Self { Self :: Mem { addr : addr . into () } } # [doc = " Asserts that in register mode, the reg class is the one that's expected."] pub (crate) fn assert_regclass_is (& self , expected_reg_class : RegClass) { if let Self :: Reg { reg } = self { debug_assert_eq ! (reg . class () , expected_reg_class) ; } } # [doc = " Add the regs mentioned by `self` to `collector`."] pub (crate) fn get_operands (& mut self , collector : & mut impl OperandVisitor) { match self { RegMem :: Reg { reg } => collector . reg_use (reg) , RegMem :: Mem { addr , .. } => addr . get_operands (collector) , } } }
};
}
