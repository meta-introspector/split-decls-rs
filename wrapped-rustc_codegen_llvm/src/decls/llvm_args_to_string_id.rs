macro_rules! llvm_args_to_string_id {
    () => {
        fn llvm_args_to_string_id (profiler : & SelfProfiler , pass_name : & str , ir_name : & str) -> EventId { let pass_name = profiler . get_or_alloc_cached_string (pass_name) ; let mut components = vec ! [StringComponent :: Ref (pass_name)] ; let parentheses : & [_] = & ['(' , ')'] ; let trimmed = ir_name . trim_matches (parentheses) ; for part in trimmed . split (", ") { let demangled_ir_name = rustc_demangle :: demangle (part) . to_string () ; let ir_name = profiler . get_or_alloc_cached_string (demangled_ir_name) ; components . push (StringComponent :: Value (SEPARATOR_BYTE)) ; components . push (StringComponent :: Ref (ir_name)) ; } EventId :: from_label (profiler . alloc_string (components . as_slice ())) }
    };
}

llvm_args_to_string_id!()