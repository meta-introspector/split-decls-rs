macro_rules! deps {
    () => {
        ModuleFlagMergeBehavior!();
    };
}

macro_rules! add_module_flag_u32 {
    () => {
        deps!();
        pub (crate) fn add_module_flag_u32 (module : & Module , merge_behavior : ModuleFlagMergeBehavior , key : & str , value : u32 ,) { unsafe { LLVMRustAddModuleFlagU32 (module , merge_behavior , key . as_c_char_ptr () , key . len () , value) ; } }
    };
}

add_module_flag_u32!();