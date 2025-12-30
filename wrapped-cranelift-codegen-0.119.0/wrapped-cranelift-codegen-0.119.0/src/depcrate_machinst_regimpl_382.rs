// Generated macro for impl_382 (impl)
macro_rules! Depcrate_machinst_regimpl_382 {
() => {
// Module: crate::machinst::reg
// Provides: {"impl_382"}
// Dependencies: {}
impl < 'a , F : Fn (VReg) -> VReg > OperandVisitor for OperandCollector < 'a , F > { fn add_operand (& mut self , reg : & mut Reg , constraint : OperandConstraint , kind : OperandKind , pos : OperandPos ,) { reg . 0 = (self . renamer) (reg . 0) ; self . operands . push (Operand :: new (reg . 0 , constraint , kind , pos)) ; } fn debug_assert_is_allocatable_preg (& self , reg : PReg , expected : bool) { debug_assert_eq ! (self . allocatable . contains (reg) , expected , "{reg:?} should{} be allocatable" , if expected { "" } else { " not" }) ; } fn reg_clobbers (& mut self , regs : PRegSet) { self . clobbers . union_from (regs) ; } }
};
}
