// Generated macro for pairmemarg_operands (function)
macro_rules! Depcrate_isa_aarch64_instpairmemarg_operands {
() => {
// Module: crate::isa::aarch64::inst
// Provides: {"pairmemarg_operands"}
// Dependencies: {}
fn pairmemarg_operands (pairmemarg : & mut PairAMode , collector : & mut impl OperandVisitor) { match pairmemarg { PairAMode :: SignedOffset { reg , .. } => { collector . reg_use (reg) ; } PairAMode :: SPPreIndexed { .. } | PairAMode :: SPPostIndexed { .. } => { } } }
};
}
