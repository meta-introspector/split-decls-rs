// Generated macro for OperandVisitor (trait)
macro_rules! Depcrate_machinst_regOperandVisitor {
() => {
// Module: crate::machinst::reg
// Provides: {"OperandVisitor"}
// Dependencies: {}
pub trait OperandVisitor { fn add_operand (& mut self , reg : & mut Reg , constraint : OperandConstraint , kind : OperandKind , pos : OperandPos ,) ; fn debug_assert_is_allocatable_preg (& self , _reg : PReg , _expected : bool) { } # [doc = " Add a register clobber set. This is a set of registers that"] # [doc = " are written by the instruction, so must be reserved (not used)"] # [doc = " for the whole instruction, but are not used afterward."] fn reg_clobbers (& mut self , _regs : PRegSet) { } }
};
}
