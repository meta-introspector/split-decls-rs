// Generated macro for setup_rustc (function)
macro_rules! Depcrate_testsetup_rustc {
() => {
// Module: crate::test
// Provides: {"setup_rustc"}
// Dependencies: {}
fn setup_rustc (env : & mut Env , args : & TestArg) -> Result < PathBuf , String > { let toolchain = format ! ("+{channel}-{host}" , channel = get_toolchain () ?, host = args . config_info . host_triple) ; let rust_dir_path = Path :: new (crate :: BUILD_DIR) . join ("rust") ; let _ = git_clone ("https://github.com/rust-lang/rust.git" , Some (& rust_dir_path) , false) ; let rust_dir : Option < & Path > = Some (& rust_dir_path) ; run_command (& [& "git" , & "checkout" , & "--" , & "tests/"] , rust_dir) ? ; run_command_with_output_and_env (& [& "git" , & "fetch"] , rust_dir , Some (env)) ? ; let rustc_commit = match rustc_version_info (env . get ("RUSTC") . map (| s | s . as_str ())) ? . commit_hash { Some (commit_hash) => commit_hash , None => return Err ("Couldn't retrieve rustc commit hash" . to_string ()) , } ; if rustc_commit != "unknown" { run_command_with_output_and_env (& [& "git" , & "checkout" , & rustc_commit] , rust_dir , Some (env) ,) ? ; } else { run_command_with_output_and_env (& [& "git" , & "checkout"] , rust_dir , Some (env)) ? ; } let cargo = String :: from_utf8 (run_command_with_env (& [& "rustup" , & "which" , & "cargo"] , rust_dir , Some (env)) ? . stdout ,) . map_err (| error | format ! ("Failed to retrieve cargo path: {error:?}")) . and_then (| cargo | { let cargo = cargo . trim () . to_owned () ; if cargo . is_empty () { Err ("`cargo` path is empty" . to_string ()) } else { Ok (cargo) } }) ? ; let rustc = String :: from_utf8 (run_command_with_env (& [& "rustup" , & toolchain , & "which" , & "rustc"] , rust_dir , Some (env)) ? . stdout ,) . map_err (| error | format ! ("Failed to retrieve rustc path: {error:?}")) . and_then (| rustc | { let rustc = rustc . trim () . to_owned () ; if rustc . is_empty () { Err ("`rustc` path is empty" . to_string ()) } else { Ok (rustc) } }) ? ; let llvm_filecheck = match run_command_with_env (& [& "bash" , & "-c" , & "which FileCheck-10 || \
          which FileCheck-11 || \
          which FileCheck-12 || \
          which FileCheck-13 || \
          which FileCheck-14 || \
          which FileCheck" ,] , rust_dir , Some (env) ,) { Ok (cmd) => String :: from_utf8_lossy (& cmd . stdout) . to_string () , Err (_) => { eprintln ! ("Failed to retrieve LLVM FileCheck, ignoring...") ; String :: new () } } ; let file_path = rust_dir_path . join ("config.toml") ; std :: fs :: write (& file_path , format ! (r#"change-id = 115898

[rust]
codegen-backends = ["gcc"]
deny-warnings = false
verbose-tests = true

[build]
cargo = "{cargo}"
local-rebuild = true
rustc = "{rustc}"

[target.x86_64-unknown-linux-gnu]
llvm-filecheck = "{llvm_filecheck}"

[llvm]
download-ci-llvm = false
"# , cargo = cargo , rustc = rustc , llvm_filecheck = llvm_filecheck . trim () ,) ,) . map_err (| error | format ! ("Failed to write into `{}`: {:?}" , file_path . display () , error)) ? ; Ok (rust_dir_path) }
};
}
