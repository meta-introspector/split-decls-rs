// Generated macro for memarg_operands (function)
macro_rules! Depcrate_isa_aarch64_instmemarg_operands {
() => {
// Module: crate::isa::aarch64::inst
// Provides: {"memarg_operands"}
// Dependencies: {}
fn memarg_operands (memarg : & mut AMode , collector : & mut impl OperandVisitor) { match memarg { AMode :: Unscaled { rn , .. } | AMode :: UnsignedOffset { rn , .. } => { collector . reg_use (rn) ; } AMode :: RegReg { rn , rm , .. } | AMode :: RegScaled { rn , rm , .. } | AMode :: RegScaledExtended { rn , rm , .. } | AMode :: RegExtended { rn , rm , .. } => { collector . reg_use (rn) ; collector . reg_use (rm) ; } AMode :: Label { .. } => { } AMode :: SPPreIndexed { .. } | AMode :: SPPostIndexed { .. } => { } AMode :: FPOffset { .. } | AMode :: IncomingArg { .. } => { } AMode :: SPOffset { .. } | AMode :: SlotOffset { .. } => { } AMode :: RegOffset { rn , .. } => { collector . reg_use (rn) ; } AMode :: Const { .. } => { } } }
};
}
