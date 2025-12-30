// Generated macro for run_first_pass (function)
macro_rules! Depcrate_firstpassrun_first_pass {
() => {
// Module: crate::firstpass
// Provides: {"run_first_pass"}
// Dependencies: {}
# [doc = " Runs the first pass, which resolves the block structure of the document,"] # [doc = " and returns the resulting tree."] pub (crate) fn run_first_pass (text : & str , options : Options) -> (Tree < Item > , Allocations < '_ >) { let start_capacity = max (128 , text . len () / 32) ; let lookup_table = & create_lut (& options) ; let first_pass = FirstPass { text , tree : Tree :: with_capacity (start_capacity) , begin_list_item : None , last_line_blank : false , allocs : Allocations :: new () , options , lookup_table , brace_context_next : 0 , brace_context_stack : Vec :: new () , } ; first_pass . run () }
};
}
