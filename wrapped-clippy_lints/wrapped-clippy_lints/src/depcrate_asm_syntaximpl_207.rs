// Generated macro for impl_207 (impl)
macro_rules! Depcrate_asm_syntaximpl_207 {
() => {
// Module: crate::asm_syntax
// Provides: {"impl_207"}
// Dependencies: {}
impl std :: ops :: Not for AsmStyle { type Output = AsmStyle ; fn not (self) -> AsmStyle { match self { AsmStyle :: Intel => AsmStyle :: Att , AsmStyle :: Att => AsmStyle :: Intel , } } }
};
}
