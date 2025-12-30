// Generated macro for test_prebuilt_llvm_config_path_resolution (function)
macro_rules! Depcrate_core_builder_teststest_prebuilt_llvm_config_path_resolution {
() => {
// Module: crate::core::builder::tests
// Provides: {"test_prebuilt_llvm_config_path_resolution"}
// Dependencies: {}
# [test] fn test_prebuilt_llvm_config_path_resolution () { fn configure (config : & str) -> Config { Config :: parse_inner (Flags :: parse (& ["build" . to_string () , "--dry-run" . to_string () , "--config=/does/not/exist" . to_string () ,]) , | & _ | toml :: from_str (& config) ,) } fn drop_win_disk_prefix_if_present (path : PathBuf) -> PathBuf { let path_str = path . to_str () . unwrap () ; if let Some ((_ , without_prefix)) = path_str . split_once (":/") { return PathBuf :: from (format ! ("/{}" , without_prefix)) ; } path } let config = configure (r#"
            [llvm]
            download-ci-llvm = false

            [build]
            build = "x86_64-unknown-linux-gnu"
            host = ["arm-unknown-linux-gnueabihf"]
            target = ["arm-unknown-linux-gnueabihf"]

            [target.x86_64-unknown-linux-gnu]
            llvm-config = "/some/path/to/llvm-config"

            [target.arm-unknown-linux-gnueabihf]
            cc = "arm-linux-gnueabihf-gcc"
            cxx = "arm-linux-gnueabihf-g++"
        "# ,) ; let build = Build :: new (config) ; let builder = Builder :: new (& build) ; let expected = PathBuf :: from ("/some/path/to/llvm-config") ; let actual = prebuilt_llvm_config (& builder , TargetSelection :: from_user ("arm-unknown-linux-gnueabihf") , false ,) . llvm_result () . host_llvm_config . clone () ; let actual = drop_win_disk_prefix_if_present (actual) ; assert_eq ! (expected , actual) ; let actual = prebuilt_llvm_config (& builder , builder . config . host_target , false) . llvm_result () . host_llvm_config . clone () ; let actual = drop_win_disk_prefix_if_present (actual) ; assert_eq ! (expected , actual) ; assert_eq ! (expected , actual) ; let config = configure (r#"
            [llvm]
            download-ci-llvm = false
        "# ,) ; let build = Build :: new (config . clone ()) ; let builder = Builder :: new (& build) ; let actual = prebuilt_llvm_config (& builder , builder . config . host_target , false) . llvm_result () . host_llvm_config . clone () ; let expected = builder . out . join (builder . config . host_target) . join ("llvm/bin") . join (exe ("llvm-config" , builder . config . host_target)) ; assert_eq ! (expected , actual) ; let config = configure (r#"
            [llvm]
            download-ci-llvm = "if-unchanged"
        "# ,) ; if config . llvm_from_ci { let build = Build :: new (config . clone ()) ; let builder = Builder :: new (& build) ; let actual = prebuilt_llvm_config (& builder , builder . config . host_target , false) . llvm_result () . host_llvm_config . clone () ; let expected = builder . out . join (builder . config . host_target) . join ("ci-llvm/bin") . join (exe ("llvm-config" , builder . config . host_target)) ; assert_eq ! (expected , actual) ; } }
};
}
