// Generated macro for override_toml (function)
macro_rules! Depcrate_core_config_testsoverride_toml {
() => {
// Module: crate::core::config::tests
// Provides: {"override_toml"}
// Dependencies: {}
# [test] fn override_toml () { let config = Config :: parse_inner (Flags :: parse (& ["check" . to_owned () , "--config=/does/not/exist" . to_owned () , "--set=change-id=1" . to_owned () , "--set=rust.lto=fat" . to_owned () , "--set=rust.deny-warnings=false" . to_owned () , "--set=build.optimized-compiler-builtins=true" . to_owned () , "--set=build.gdb=\"bar\"" . to_owned () , "--set=build.tools=[\"cargo\"]" . to_owned () , "--set=llvm.build-config={\"foo\" = \"bar\"}" . to_owned () , "--set=target.x86_64-unknown-linux-gnu.runner=bar" . to_owned () , "--set=target.x86_64-unknown-linux-gnu.rpath=false" . to_owned () , "--set=target.aarch64-unknown-linux-gnu.sanitizers=false" . to_owned () , "--set=target.aarch64-apple-darwin.runner=apple" . to_owned () , "--set=target.aarch64-apple-darwin.optimized-compiler-builtins=false" . to_owned () ,]) , | & _ | { toml :: from_str (r#"
change-id = 0
[rust]
lto = "off"
deny-warnings = true
download-rustc=false

[build]
gdb = "foo"
tools = []

[llvm]
download-ci-llvm = false
build-config = {}

[target.aarch64-unknown-linux-gnu]
sanitizers = true
rpath = true
runner = "aarch64-runner"

[target.x86_64-unknown-linux-gnu]
sanitizers = true
rpath = true
runner = "x86_64-runner"

                "# ,) } ,) ; assert_eq ! (config . change_id , Some (ChangeId :: Id (1)) , "setting top-level value") ; assert_eq ! (config . rust_lto , crate :: core :: config :: RustcLto :: Fat , "setting string value without quotes") ; assert_eq ! (config . gdb , Some ("bar" . into ()) , "setting string value with quotes") ; assert ! (! config . deny_warnings , "setting boolean value") ; assert_eq ! (config . optimized_compiler_builtins , CompilerBuiltins :: BuildLLVMFuncs , "setting boolean value") ; assert_eq ! (config . tools , Some (["cargo" . to_string ()] . into_iter () . collect ()) , "setting list value") ; assert_eq ! (config . llvm_build_config , [("foo" . to_string () , "bar" . to_string ())] . into_iter () . collect () , "setting dictionary value") ; let x86_64 = TargetSelection :: from_user ("x86_64-unknown-linux-gnu") ; let x86_64_values = Target { sanitizers : Some (true) , rpath : Some (false) , runner : Some ("bar" . into ()) , .. Default :: default () } ; let aarch64 = TargetSelection :: from_user ("aarch64-unknown-linux-gnu") ; let aarch64_values = Target { sanitizers : Some (false) , rpath : Some (true) , runner : Some ("aarch64-runner" . into ()) , .. Default :: default () } ; let darwin = TargetSelection :: from_user ("aarch64-apple-darwin") ; let darwin_values = Target { runner : Some ("apple" . into ()) , optimized_compiler_builtins : Some (CompilerBuiltins :: BuildRustOnly) , .. Default :: default () } ; assert_eq ! (config . target_config , [(x86_64 , x86_64_values) , (aarch64 , aarch64_values) , (darwin , darwin_values)] . into_iter () . collect () , "setting dictionary value") ; assert ! (! config . llvm_from_ci) ; assert ! (! config . download_rustc ()) ; }
};
}
