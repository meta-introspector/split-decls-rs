macro_rules! macro_0 {
    () => {
        # [cfg (all (procmacro2_semver_exempt , wrap_proc_macro , not (super_unstable)))] compile_error ! { "\
    Something is not right. If you've tried to turn on \
    procmacro2_semver_exempt, you need to ensure that it \
    is turned on for the compilation of the proc-macro2 \
    build script as well.
" }
    };
}

macro_0!()