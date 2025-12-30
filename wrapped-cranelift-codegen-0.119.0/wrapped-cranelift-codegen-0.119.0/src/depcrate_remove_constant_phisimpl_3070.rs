// Generated macro for impl_3070 (impl)
macro_rules! Depcrate_remove_constant_phisimpl_3070 {
() => {
// Module: crate::remove_constant_phis
// Provides: {"impl_3070"}
// Dependencies: {}
impl < 'a > OutEdge < 'a > { # [doc = " Construct a new `OutEdge` for the given instruction."] # [doc = ""] # [doc = " Returns `None` if this is an edge without any block arguments, which"] # [doc = " means we can ignore it for this analysis's purposes."] # [inline] fn new (bump : & 'a Bump , dfg : & ir :: DataFlowGraph , inst : Inst , branch_index : usize , block : BlockCall ,) -> Option < Self > { let inst_var_args = block . args_slice (& dfg . value_lists) ; if inst_var_args . is_empty () { return None ; } Some (OutEdge { inst , branch_index : branch_index as u32 , block : block . block (& dfg . value_lists) , args : bump . alloc_slice_fill_iter (inst_var_args . iter () . map (| value | dfg . resolve_aliases (* value)) ,) , }) } }
};
}
