// Generated macro for write_user_stack_map_entries (function)
macro_rules! Depcrate_writewrite_user_stack_map_entries {
() => {
// Module: crate::write
// Provides: {"write_user_stack_map_entries"}
// Dependencies: {}
fn write_user_stack_map_entries (w : & mut dyn Write , dfg : & DataFlowGraph , inst : Inst) -> fmt :: Result { let entries = match dfg . user_stack_map_entries (inst) { None => return Ok (()) , Some (es) => es , } ; write ! (w , ", stack_map=[") ? ; let mut need_comma = false ; for entry in entries { if need_comma { write ! (w , ", ") ? ; } write ! (w , "{} @ {}+{}" , entry . ty , entry . slot , entry . offset) ? ; need_comma = true ; } write ! (w , "]") ? ; Ok (()) }
};
}
