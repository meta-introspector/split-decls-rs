// Generated macro for run_tests (function)
macro_rules! Depcrate_testsrun_tests {
() => {
// Module: crate::tests
// Provides: {"run_tests"}
// Dependencies: {}
# [doc = " Run tests on optimized dist artifacts."] pub fn run_tests (env : & Environment) -> anyhow :: Result < () > { let build_dir = env . build_root () ; let dist_dir = build_dir . join ("dist") ; let unpacked_dist_dir = build_dir . join ("unpacked-dist") ; std :: fs :: create_dir_all (& unpacked_dist_dir) ? ; let extract_dist_dir = | name : & str | -> anyhow :: Result < Utf8PathBuf > { unpack_archive (& dist_dir . join (format ! ("{name}.tar.xz")) , & unpacked_dist_dir) ? ; let extracted_path = unpacked_dist_dir . join (name) ; assert ! (extracted_path . is_dir ()) ; Ok (extracted_path) } ; let host_triple = env . host_tuple () ; let version = find_dist_version (& dist_dir) ? ; let channel = version_to_channel (& version) ; let rustc_dir = extract_dist_dir (& format ! ("rustc-{version}-{host_triple}")) ? . join ("rustc") ; let libstd_dir = extract_dist_dir (& format ! ("rust-std-{version}-{host_triple}")) ? . join (format ! ("rust-std-{host_triple}")) ; let cargo_dir = extract_dist_dir (& format ! ("cargo-{version}-{host_triple}")) ? . join ("cargo") ; let extracted_src_dir = extract_dist_dir (& format ! ("rust-src-{version}")) ? . join ("rust-src") ; if let Ok (_) = find_file_in_dir (& dist_dir , "rustc-codegen-cranelift-" , ".tar.xz") { let extracted_codegen_dir = extract_dist_dir (& format ! ("rustc-codegen-cranelift-{version}-{host_triple}")) ? . join ("rustc-codegen-cranelift-preview") ; let rel_path = Utf8Path :: new ("lib") . join ("rustlib") . join (host_triple) . join ("codegen-backends") ; copy_directory (& extracted_codegen_dir . join (& rel_path) , & rustc_dir . join (& rel_path)) ? ; } copy_directory (& libstd_dir . join ("lib") . join ("rustlib") . join (host_triple) . join ("lib") , & rustc_dir . join ("lib") . join ("rustlib") . join (host_triple) . join ("lib") ,) ? ; copy_directory (& extracted_src_dir . join ("lib") . join ("rustlib") . join ("src") , & rustc_dir . join ("lib") . join ("rustlib") . join ("src") ,) ? ; let rustc_path = rustc_dir . join ("bin") . join (format ! ("rustc{}" , executable_extension ())) ; assert ! (rustc_path . is_file ()) ; let cargo_path = cargo_dir . join ("bin") . join (format ! ("cargo{}" , executable_extension ())) ; assert ! (cargo_path . is_file ()) ; let llvm_config = env . build_artifacts () . join ("llvm") . join ("bin") . join (format ! ("llvm-config{}" , executable_extension ())) ; assert ! (llvm_config . is_file ()) ; let config_content = format ! (r#"
profile = "user"
change-id = 115898

[rust]
channel = "{channel}"
verbose-tests = true
# rust-lld cannot be combined with an external LLVM
lld = false

[build]
rustc = "{rustc}"
cargo = "{cargo}"
local-rebuild = true
compiletest-allow-stage0=true

[target.{host_triple}]
llvm-config = "{llvm_config}"
"# , rustc = rustc_path . to_string () . replace ('\\' , "/") , cargo = cargo_path . to_string () . replace ('\\' , "/") , llvm_config = llvm_config . to_string () . replace ('\\' , "/")) ; log :: info ! ("Using following `bootstrap.toml` for running tests:\n{config_content}") ; with_backed_up_file (Path :: new ("bootstrap.toml") , & config_content , | | { let x_py = env . checkout_path () . join ("x.py") ; let mut args = vec ! [env . python_binary () , x_py . as_str () , "test" , "--build" , env . host_tuple () , "--stage" , "0" , "tests/assembly-llvm" , "tests/codegen-llvm" , "tests/codegen-units" , "tests/incremental" , "tests/mir-opt" , "tests/pretty" , "tests/run-make/glibc-symbols-x86_64-unknown-linux-gnu" , "tests/run-make/rust-lld-x86_64-unknown-linux-gnu-dist" , "tests/ui" , "tests/crashes" ,] ; for test_path in env . skipped_tests () { args . extend (["--skip" , test_path]) ; } cmd (& args) . env ("COMPILETEST_ENABLE_DIST_TESTS" , "1") . run () . context ("Cannot execute tests") }) }
};
}
