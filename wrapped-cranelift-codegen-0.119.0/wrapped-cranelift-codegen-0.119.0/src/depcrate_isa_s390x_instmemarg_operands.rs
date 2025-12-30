// Generated macro for memarg_operands (function)
macro_rules! Depcrate_isa_s390x_instmemarg_operands {
() => {
// Module: crate::isa::s390x::inst
// Provides: {"memarg_operands"}
// Dependencies: {}
fn memarg_operands (memarg : & mut MemArg , collector : & mut impl OperandVisitor) { match memarg { MemArg :: BXD12 { base , index , .. } | MemArg :: BXD20 { base , index , .. } => { collector . reg_use (base) ; collector . reg_use (index) ; } MemArg :: Label { .. } | MemArg :: Symbol { .. } => { } MemArg :: RegOffset { reg , .. } => { collector . reg_use (reg) ; } MemArg :: InitialSPOffset { .. } | MemArg :: NominalSPOffset { .. } | MemArg :: SlotOffset { .. } => { } } collector . reg_fixed_nonallocatable (gpr_preg (1)) ; }
};
}
