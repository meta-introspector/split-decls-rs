// Generated macro for impl_376 (impl)
macro_rules! Depcrate_hirimpl_376 {
() => {
// Module: crate::hir
// Provides: {"impl_376"}
// Dependencies: {}
impl < 'hir > InlineAsmOperand < 'hir > { pub fn reg (& self) -> Option < InlineAsmRegOrRegClass > { match * self { Self :: In { reg , .. } | Self :: Out { reg , .. } | Self :: InOut { reg , .. } | Self :: SplitInOut { reg , .. } => Some (reg) , Self :: Const { .. } | Self :: SymFn { .. } | Self :: SymStatic { .. } | Self :: Label { .. } => None , } } pub fn is_clobber (& self) -> bool { matches ! (self , InlineAsmOperand :: Out { reg : InlineAsmRegOrRegClass :: Reg (_) , late : _ , expr : None }) } }
};
}
