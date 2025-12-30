// Generated macro for has_memory_fence_semantics (function)
macro_rules! Depcrate_inst_predicateshas_memory_fence_semantics {
() => {
// Module: crate::inst_predicates
// Provides: {"has_memory_fence_semantics"}
// Dependencies: {}
# [doc = " Determine whether this opcode behaves as a memory fence, i.e.,"] # [doc = " prohibits any moving of memory accesses across it."] pub fn has_memory_fence_semantics (op : Opcode) -> bool { match op { Opcode :: AtomicRmw | Opcode :: AtomicCas | Opcode :: AtomicLoad | Opcode :: AtomicStore | Opcode :: Fence | Opcode :: Debugtrap => true , Opcode :: Call | Opcode :: CallIndirect => true , op if op . can_trap () => true , _ => false , } }
};
}
