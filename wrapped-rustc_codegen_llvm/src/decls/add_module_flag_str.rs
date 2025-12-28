macro_rules! deps {
    () => {
        ModuleFlagMergeBehavior!();
    };
}

macro_rules! add_module_flag_str {
    () => {
        deps!();
        pub (crate) fn add_module_flag_str (module : & Module , merge_behavior : ModuleFlagMergeBehavior , key : & str , value : & str ,) { unsafe { LLVMRustAddModuleFlagString (module , merge_behavior , key . as_c_char_ptr () , key . len () , value . as_c_char_ptr () , value . len () ,) ; } }
    };
}

add_module_flag_str!();