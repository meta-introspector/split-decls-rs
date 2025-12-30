// Generated macro for impl_2462 (impl)
macro_rules! Depcrate_isa_s390x_instimpl_2462 {
() => {
// Module: crate::isa::s390x::inst
// Provides: {"impl_2462"}
// Dependencies: {}
impl < T : OperandVisitor > OperandVisitor for DenyReuseVisitor < '_ , T > { fn add_operand (& mut self , reg : & mut Reg , constraint : regalloc2 :: OperandConstraint , kind : regalloc2 :: OperandKind , pos : regalloc2 :: OperandPos ,) { debug_assert ! (! self . deny_reuse || ! matches ! (constraint , regalloc2 :: OperandConstraint :: Reuse (_))) ; self . inner . add_operand (reg , constraint , kind , pos) ; } fn debug_assert_is_allocatable_preg (& self , reg : regalloc2 :: PReg , expected : bool) { self . inner . debug_assert_is_allocatable_preg (reg , expected) ; } fn reg_clobbers (& mut self , regs : PRegSet) { self . inner . reg_clobbers (regs) ; } }
};
}
