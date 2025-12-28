macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! needs_gdb_debug_scripts_section {
    () => {
        deps!();
        pub (crate) fn needs_gdb_debug_scripts_section (cx : & CodegenCx < '_ , '_ >) -> bool { let embed_visualizers = cx . tcx . crate_types () . iter () . any (| & crate_type | match crate_type { CrateType :: Executable | CrateType :: Dylib | CrateType :: Cdylib | CrateType :: Staticlib | CrateType :: Sdylib => { true } CrateType :: ProcMacro => { false } CrateType :: Rlib => { false } }) ; cx . sess () . opts . debuginfo != DebugInfo :: None && cx . sess () . target . emit_debug_gdb_scripts && embed_visualizers }
    };
}

needs_gdb_debug_scripts_section!();