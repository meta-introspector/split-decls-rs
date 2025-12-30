// Generated macro for impl_9345 (impl)
macro_rules! Depcrate_returnsimpl_9345 {
() => {
// Module: crate::returns
// Provides: {"impl_9345"}
// Dependencies: {}
impl RetReplacement < '_ > { fn sugg_help (& self) -> & 'static str { match self { Self :: Empty | Self :: Expr (..) => "remove `return`" , Self :: Block => "replace `return` with an empty block" , Self :: Unit => "replace `return` with a unit value" , Self :: NeedsPar (..) => "remove `return` and wrap the sequence with parentheses" , } } fn applicability (& self) -> Applicability { match self { Self :: Expr (_ , ap) | Self :: NeedsPar (_ , ap) => * ap , _ => Applicability :: MachineApplicable , } } }
};
}
