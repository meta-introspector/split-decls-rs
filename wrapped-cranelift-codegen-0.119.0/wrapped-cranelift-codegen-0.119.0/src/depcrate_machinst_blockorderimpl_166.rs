// Generated macro for impl_166 (impl)
macro_rules! Depcrate_machinst_blockorderimpl_166 {
() => {
// Module: crate::machinst::blockorder
// Provides: {"impl_166"}
// Dependencies: {}
impl LoweredBlock { # [doc = " Unwrap an `Orig` block."] pub fn orig_block (& self) -> Option < Block > { match self { & LoweredBlock :: Orig { block } => Some (block) , & LoweredBlock :: CriticalEdge { .. } => None , } } # [doc = " The associated in-edge predecessor, if this is a critical edge."] # [cfg (test)] pub fn in_edge (& self) -> Option < Block > { match self { & LoweredBlock :: CriticalEdge { pred , .. } => Some (pred) , & LoweredBlock :: Orig { .. } => None , } } # [doc = " The associated out-edge successor, if this is a critical edge."] pub fn out_edge (& self) -> Option < Block > { match self { & LoweredBlock :: CriticalEdge { succ , .. } => Some (succ) , & LoweredBlock :: Orig { .. } => None , } } }
};
}
