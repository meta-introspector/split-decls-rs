// Generated macro for TestArg (struct)
macro_rules! Depcrate_testTestArg {
() => {
// Module: crate::test
// Provides: {"TestArg"}
// Dependencies: {}
# [derive (Default , Debug)] struct TestArg { build_only : bool , use_system_gcc : bool , runners : Vec < String > , flags : Vec < String > , # [doc = " Additional arguments, to be passed to commands like `cargo test`."] test_args : Vec < String > , nb_parts : Option < usize > , current_part : Option < usize > , sysroot_panic_abort : bool , config_info : ConfigInfo , sysroot_features : Vec < String > , keep_lto_tests : bool , }
};
}
