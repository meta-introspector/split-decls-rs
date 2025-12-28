macro_rules! deps {
    () => {
        ModuleLlvm!();
    };
}

macro_rules! write_bitcode_to_file {
    () => {
        deps!();
        fn write_bitcode_to_file (module : & ModuleCodegen < ModuleLlvm > , path : & Path) { unsafe { let path = path_to_c_string (& path) ; let llmod = module . module_llvm . llmod () ; llvm :: LLVMWriteBitcodeToFile (llmod , path . as_ptr ()) ; } }
    };
}

write_bitcode_to_file!();