// Generated macro for Legalize (type)
macro_rules! Depcrate_isaLegalize {
() => {
// Module: crate::isa
// Provides: {"Legalize"}
// Dependencies: {}
# [doc = " After determining that an instruction doesn't have an encoding, how should we proceed to"] # [doc = " legalize it?"] # [doc = ""] # [doc = " The `Encodings` iterator returns a legalization function to call."] pub type Legalize = fn (ir :: Inst , & mut ir :: Function , & mut flowgraph :: ControlFlowGraph , & dyn TargetIsa) -> bool ;
};
}
