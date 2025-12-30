// Generated macro for impl_1757 (impl)
macro_rules! Depcrate_isa_aarch64_inst_argsimpl_1757 {
() => {
// Module: crate::isa::aarch64::inst::args
// Provides: {"impl_1757"}
// Dependencies: {}
impl TestBitAndBranchKind { # [doc = " Complements this branch condition to act on the opposite result."] pub fn complement (& self) -> TestBitAndBranchKind { match self { TestBitAndBranchKind :: Z => TestBitAndBranchKind :: NZ , TestBitAndBranchKind :: NZ => TestBitAndBranchKind :: Z , } } }
};
}
