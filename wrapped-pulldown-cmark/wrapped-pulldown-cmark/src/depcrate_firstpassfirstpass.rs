// Generated macro for FirstPass (struct)
macro_rules! Depcrate_firstpassFirstPass {
() => {
// Module: crate::firstpass
// Provides: {"FirstPass"}
// Dependencies: {}
# [doc = " State for the first parsing pass."] struct FirstPass < 'a , 'b > { text : & 'a str , tree : Tree < Item > , begin_list_item : Option < usize > , last_line_blank : bool , allocs : Allocations < 'a > , options : Options , lookup_table : & 'b LookupTable , # [doc = " Math environment brace nesting."] brace_context_stack : Vec < u8 > , brace_context_next : usize , }
};
}
