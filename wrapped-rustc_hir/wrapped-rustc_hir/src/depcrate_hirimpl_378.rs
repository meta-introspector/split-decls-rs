// Generated macro for impl_378 (impl)
macro_rules! Depcrate_hirimpl_378 {
() => {
// Module: crate::hir
// Provides: {"impl_378"}
// Dependencies: {}
impl InlineAsm < '_ > { pub fn contains_label (& self) -> bool { self . operands . iter () . any (| x | matches ! (x . 0 , InlineAsmOperand :: Label { .. })) } }
};
}
