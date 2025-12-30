// Generated macro for eliminate_unreachable_code (function)
macro_rules! Depcrate_unreachable_codeeliminate_unreachable_code {
() => {
// Module: crate::unreachable_code
// Provides: {"eliminate_unreachable_code"}
// Dependencies: {}
# [doc = " Eliminate unreachable code."] # [doc = ""] # [doc = " This pass deletes whole blocks that can't be reached from the entry block. It does not delete"] # [doc = " individual instructions whose results are unused."] # [doc = ""] # [doc = " The reachability analysis is performed by the dominator tree analysis."] pub fn eliminate_unreachable_code (func : & mut ir :: Function , cfg : & mut ControlFlowGraph , domtree : & DominatorTree ,) { let _tt = timing :: unreachable_code () ; let mut pos = FuncCursor :: new (func) ; let mut used_tables = EntitySet :: with_capacity (pos . func . stencil . dfg . jump_tables . len ()) ; while let Some (block) = pos . next_block () { if domtree . is_reachable (block) { let inst = pos . func . layout . last_inst (block) . unwrap () ; if let ir :: InstructionData :: BranchTable { table , .. } = pos . func . dfg . insts [inst] { used_tables . insert (table) ; } continue ; } trace ! ("Eliminating unreachable {}" , block) ; pos . prev_block () ; while let Some (inst) = pos . func . layout . first_inst (block) { trace ! (" - {}" , pos . func . dfg . display_inst (inst)) ; pos . func . layout . remove_inst (inst) ; } cfg . recompute_block (pos . func , block) ; pos . func . layout . remove_block (block) ; } for (table , jt_data) in func . stencil . dfg . jump_tables . iter_mut () { if ! used_tables . contains (table) { jt_data . clear () ; } } }
};
}
