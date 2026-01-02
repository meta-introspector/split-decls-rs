mkuse!{use rustc_index :: bit_set :: DenseBitSet ;}
mkuse!{use super :: * ;}

macro_rules! maybe_loop_headers_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function maybe_loop_headers in module {}", module_path!());
    };
}

mkfn!{
    maybe_loop_headers_introspect!();
    # [doc = " Compute the set of loop headers in the given body. A loop header is usually defined as a block"] # [doc = " which dominates one of its predecessors. This definition is only correct for reducible CFGs."] # [doc = " However, computing dominators is expensive, so we approximate according to the post-order"] # [doc = " traversal order. A loop header for us is a block which is visited after its predecessor in"] # [doc = " post-order. This is ok as we mostly need a heuristic."] pub fn maybe_loop_headers (body : & Body < '_ >) -> DenseBitSet < BasicBlock > { let mut maybe_loop_headers = DenseBitSet :: new_empty (body . basic_blocks . len ()) ; let mut visited = DenseBitSet :: new_empty (body . basic_blocks . len ()) ; for (bb , bbdata) in traversal :: postorder (body) { for succ in bbdata . terminator () . successors () { if ! visited . contains (succ) { maybe_loop_headers . insert (succ) ; } } let _new = visited . insert (bb) ; debug_assert ! (_new) ; } maybe_loop_headers }
}