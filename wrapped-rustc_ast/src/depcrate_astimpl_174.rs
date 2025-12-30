// Generated macro for impl_174 (impl)
macro_rules! Depcrate_astimpl_174 {
() => {
// Module: crate::ast
// Provides: {"impl_174"}
// Dependencies: {}
impl InlineAsmOperand { pub fn reg (& self) -> Option < & InlineAsmRegOrRegClass > { match self { Self :: In { reg , .. } | Self :: Out { reg , .. } | Self :: InOut { reg , .. } | Self :: SplitInOut { reg , .. } => Some (reg) , Self :: Const { .. } | Self :: Sym { .. } | Self :: Label { .. } => None , } } }
};
}
