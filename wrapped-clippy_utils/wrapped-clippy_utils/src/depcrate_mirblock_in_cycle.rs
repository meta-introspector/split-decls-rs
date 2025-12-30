// Generated macro for block_in_cycle (function)
macro_rules! Depcrate_mirblock_in_cycle {
() => {
// Module: crate::mir
// Provides: {"block_in_cycle"}
// Dependencies: {}
# [doc = " Checks if the block is part of a cycle"] pub fn block_in_cycle (body : & Body < '_ > , block : BasicBlock) -> bool { let mut seen = DenseBitSet :: new_empty (body . basic_blocks . len ()) ; let mut to_visit = Vec :: with_capacity (body . basic_blocks . len () / 2) ; seen . insert (block) ; let mut next = block ; loop { for succ in body . basic_blocks [next] . terminator () . successors () { if seen . insert (succ) { to_visit . push (succ) ; } else if succ == block { return true ; } } if let Some (x) = to_visit . pop () { next = x ; } else { return false ; } } }
};
}
