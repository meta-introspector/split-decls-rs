// Generated macro for InstColor (struct)
macro_rules! Depcrate_machinst_lowerInstColor {
() => {
// Module: crate::machinst::lower
// Provides: {"InstColor"}
// Dependencies: {}
# [doc = " An \"instruction color\" partitions CLIF instructions by side-effecting ops."] # [doc = " All instructions with the same \"color\" are guaranteed not to be separated by"] # [doc = " any side-effecting op (for this purpose, loads are also considered"] # [doc = " side-effecting, to avoid subtle questions w.r.t. the memory model), and"] # [doc = " furthermore, it is guaranteed that for any two instructions A and B such"] # [doc = " that color(A) == color(B), either A dominates B and B postdominates A, or"] # [doc = " vice-versa. (For now, in practice, only ops in the same basic block can ever"] # [doc = " have the same color, trivially providing the second condition.) Intuitively,"] # [doc = " this means that the ops of the same color must always execute \"together\", as"] # [doc = " part of one atomic contiguous section of the dynamic execution trace, and"] # [doc = " they can be freely permuted (modulo true dataflow dependencies) without"] # [doc = " affecting program behavior."] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] struct InstColor (u32) ;
};
}
