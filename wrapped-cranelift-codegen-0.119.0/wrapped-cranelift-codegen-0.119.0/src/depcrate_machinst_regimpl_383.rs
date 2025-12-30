// Generated macro for impl_383 (impl)
macro_rules! Depcrate_machinst_regimpl_383 {
() => {
// Module: crate::machinst::reg
// Provides: {"impl_383"}
// Dependencies: {}
impl < T : FnMut (& mut Reg , OperandConstraint , OperandKind , OperandPos) > OperandVisitor for T { fn add_operand (& mut self , reg : & mut Reg , constraint : OperandConstraint , kind : OperandKind , pos : OperandPos ,) { self (reg , constraint , kind , pos) } }
};
}
