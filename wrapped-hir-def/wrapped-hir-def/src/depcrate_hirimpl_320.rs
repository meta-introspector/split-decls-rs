// Generated macro for impl_320 (impl)
macro_rules! Depcrate_hirimpl_320 {
() => {
// Module: crate::hir
// Provides: {"impl_320"}
// Dependencies: {}
impl AsmOperand { pub fn reg (& self) -> Option < & InlineAsmRegOrRegClass > { match self { Self :: In { reg , .. } | Self :: Out { reg , .. } | Self :: InOut { reg , .. } | Self :: SplitInOut { reg , .. } => Some (reg) , Self :: Const { .. } | Self :: Sym { .. } | Self :: Label { .. } => None , } } pub fn is_clobber (& self) -> bool { matches ! (self , AsmOperand :: Out { reg : InlineAsmRegOrRegClass :: Reg (_) , late : _ , expr : None }) } }
};
}
