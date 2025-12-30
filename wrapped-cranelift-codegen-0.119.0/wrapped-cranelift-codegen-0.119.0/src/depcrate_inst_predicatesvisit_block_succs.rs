// Generated macro for visit_block_succs (function)
macro_rules! Depcrate_inst_predicatesvisit_block_succs {
() => {
// Module: crate::inst_predicates
// Provides: {"visit_block_succs"}
// Dependencies: {}
# [doc = " Visit all successors of a block with a given visitor closure. The closure"] # [doc = " arguments are the branch instruction that is used to reach the successor,"] # [doc = " the successor block itself, and a flag indicating whether the block is"] # [doc = " branched to via a table entry."] pub (crate) fn visit_block_succs < F : FnMut (Inst , Block , bool) > (f : & Function , block : Block , mut visit : F ,) { if let Some (inst) = f . layout . last_inst (block) { match & f . dfg . insts [inst] { ir :: InstructionData :: Jump { destination : dest , .. } => { visit (inst , dest . block (& f . dfg . value_lists) , false) ; } ir :: InstructionData :: Brif { blocks : [block_then , block_else] , .. } => { visit (inst , block_then . block (& f . dfg . value_lists) , false) ; visit (inst , block_else . block (& f . dfg . value_lists) , false) ; } ir :: InstructionData :: BranchTable { table , .. } => { let pool = & f . dfg . value_lists ; let table = & f . stencil . dfg . jump_tables [* table] ; visit (inst , table . default_block () . block (pool) , false) ; for dest in table . as_slice () { visit (inst , dest . block (pool) , true) ; } } inst => debug_assert ! (! inst . opcode () . is_branch ()) , } } }
};
}
