// Generated macro for compile_rust_programs (function)
macro_rules! Depcrate_common_gen_rustcompile_rust_programs {
() => {
// Module: crate::common::gen_rust
// Provides: {"compile_rust_programs"}
// Dependencies: {}
pub fn compile_rust_programs (toolchain : Option < & str > , target : & str , linker : Option < & str >) -> bool { if toolchain . is_none () { return true ; } trace ! ("Building cargo command") ; let mut cargo_command = Command :: new ("cargo") ; cargo_command . current_dir ("rust_programs") ; cargo_command . env ("CARGO_TARGET_DIR" , "target") ; if toolchain . is_some_and (| val | ! val . is_empty ()) { cargo_command . arg (toolchain . unwrap ()) ; } cargo_command . args (["build" , "--target" , target , "--release"]) ; let mut rust_flags = "-Cdebuginfo=0" . to_string () ; if let Some (linker) = linker { rust_flags . push_str (" -C linker=") ; rust_flags . push_str (linker) ; rust_flags . push_str (" -C link-args=-static") ; cargo_command . env ("CPPFLAGS" , "-fuse-ld=lld") ; } cargo_command . env ("RUSTFLAGS" , rust_flags) ; trace ! ("running cargo") ; if log :: log_enabled ! (log :: Level :: Trace) { cargo_command . stdout (std :: process :: Stdio :: inherit ()) ; cargo_command . stderr (std :: process :: Stdio :: inherit ()) ; } let output = cargo_command . output () ; trace ! ("cargo is done") ; if let Ok (output) = output { if output . status . success () { true } else { error ! ("Failed to compile code for rust intrinsics\n\nstdout:\n{}\n\nstderr:\n{}" , std :: str :: from_utf8 (& output . stdout) . unwrap_or ("") , std :: str :: from_utf8 (& output . stderr) . unwrap_or ("")) ; false } } else { error ! ("Command failed: {output:#?}") ; false } }
};
}
