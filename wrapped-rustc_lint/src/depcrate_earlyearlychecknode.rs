// Generated macro for EarlyCheckNode (trait)
macro_rules! Depcrate_earlyEarlyCheckNode {
() => {
// Module: crate::early
// Provides: {"EarlyCheckNode"}
// Dependencies: {}
# [doc = " Early lints work on different nodes - either on the crate root, or on freshly loaded modules."] # [doc = " This trait generalizes over those nodes."] pub trait EarlyCheckNode < 'a > : Copy { fn id (self) -> ast :: NodeId ; fn attrs (self) -> & 'a [ast :: Attribute] ; fn check < 'ecx , 'tcx , T : EarlyLintPass > (self , cx : & mut EarlyContextAndPass < 'ecx , 'tcx , T >) ; }
};
}
