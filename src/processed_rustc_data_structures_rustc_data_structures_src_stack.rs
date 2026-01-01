/* FP:stack.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_stack_CONST_0001
/* FP:stack.rs-0002 */ const RED_ZONE : usize = 100 * 1024 ;
/* FP:stack.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_stack_CONST_0002
/* FP:stack.rs-0004 */ # [cfg (not (target_os = "aix"))] const STACK_PER_RECURSION : usize = 1024 * 1024 ;
/* FP:stack.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_stack_CONST_0003
/* FP:stack.rs-0006 */ # [cfg (target_os = "aix")] const STACK_PER_RECURSION : usize = 16 * 1024 * 1024 ;
/* FP:stack.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_stack_FN_0004
/* FP:stack.rs-0008 */ # [doc = " Grows the stack on demand to prevent stack overflow. Call this in strategic locations"] # [doc = " to \"break up\" recursive calls. E.g. almost any call to `visit_expr` or equivalent can benefit"] # [doc = " from this."] # [doc = ""] # [doc = " Should not be sprinkled around carelessly, as it causes a little bit of overhead."] # [inline] pub fn ensure_sufficient_stack < R > (f : impl FnOnce () -> R) -> R { stacker :: maybe_grow (RED_ZONE , STACK_PER_RECURSION , f) }