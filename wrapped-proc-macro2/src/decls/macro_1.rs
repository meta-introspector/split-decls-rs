macro_rules! macro_1 {
    () => {
        # [cfg (all (procmacro2_nightly_testing , feature = "proc-macro" , not (proc_macro_span)))] compile_error ! { "\
    Build script probe failed to compile.
" }
    };
}

macro_1!();