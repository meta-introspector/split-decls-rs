macro_rules! deps {
    () => {
        LlvmCodegenBackend!();
        ModuleLlvm!();
    };
}

macro_rules! save_temp_bitcode {
    () => {
        deps!();
        pub (crate) fn save_temp_bitcode (cgcx : & CodegenContext < LlvmCodegenBackend > , module : & ModuleCodegen < ModuleLlvm > , name : & str ,) { if ! cgcx . save_temps { return ; } let ext = format ! ("{name}.bc") ; let path = cgcx . output_filenames . temp_path_ext_for_cgu (& ext , & module . name , cgcx . invocation_temp . as_deref () ,) ; write_bitcode_to_file (module , & path) }
    };
}

save_temp_bitcode!()