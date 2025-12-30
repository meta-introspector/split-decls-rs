// Generated macro for test (module)
macro_rules! Depcrate_isa_riscv64_insttest {
() => {
// Module: crate::isa::riscv64::inst
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn label_use_max_range () { assert ! (LabelUse :: B12 . max_neg_range () == LabelUse :: B12 . max_pos_range () + 2) ; assert ! (LabelUse :: Jal20 . max_neg_range () == LabelUse :: Jal20 . max_pos_range () + 2) ; assert ! (LabelUse :: PCRel32 . max_pos_range () == (Inst :: imm_max () as CodeOffset)) ; assert ! (LabelUse :: PCRel32 . max_neg_range () == (Inst :: imm_min () . abs () as CodeOffset)) ; assert ! (LabelUse :: B12 . max_pos_range () == ((1 << 11) - 1) * 2) ; } }
};
}
