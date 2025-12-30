// Generated macro for OutEdge (struct)
macro_rules! Depcrate_remove_constant_phisOutEdge {
() => {
// Module: crate::remove_constant_phis
// Provides: {"OutEdge"}
// Dependencies: {}
# [derive (Clone , Copy , Debug)] struct OutEdge < 'a > { # [doc = " An instruction that transfers control."] inst : Inst , # [doc = " The index into branch_destinations for this instruction that corresponds"] # [doc = " to this edge."] branch_index : u32 , # [doc = " The block that control is transferred to."] block : Block , # [doc = " The arguments to that block."] # [doc = ""] # [doc = " These values can be from both groups A and B."] args : & 'a [Value] , }
};
}
