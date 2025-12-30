// Generated macro for build_codegen (function)
macro_rules! Depcrate_buildbuild_codegen {
() => {
// Module: crate::build
// Provides: {"build_codegen"}
// Dependencies: {}
fn build_codegen (args : & mut BuildArg) -> Result < () , String > { let mut env = HashMap :: new () ; let gcc_path = args . config_info . gcc_path . clone () . expect ("The config module should have emitted an error if the GCC path wasn't provided" ,) ; env . insert ("LD_LIBRARY_PATH" . to_string () , gcc_path . clone ()) ; env . insert ("LIBRARY_PATH" . to_string () , gcc_path) ; if args . config_info . no_default_features { env . insert ("RUSTFLAGS" . to_string () , "-Csymbol-mangling-version=v0" . to_string ()) ; } let mut command : Vec < & dyn AsRef < OsStr > > = vec ! [& "cargo" , & "rustc"] ; if args . config_info . channel == Channel :: Release { command . push (& "--release") ; env . insert ("CHANNEL" . to_string () , "release" . to_string ()) ; env . insert ("CARGO_INCREMENTAL" . to_string () , "1" . to_string ()) ; } else { env . insert ("CHANNEL" . to_string () , "debug" . to_string ()) ; } if args . config_info . no_default_features { command . push (& "--no-default-features") ; } let flags = args . flags . iter () . map (| s | s . as_str ()) . collect :: < Vec < _ > > () ; for flag in & flags { command . push (flag) ; } run_command_with_output_and_env (& command , None , Some (& env)) ? ; args . config_info . setup (& mut env , false) ? ; let _ = fs :: remove_dir_all ("target/out") ; let gccjit_target = "target/out/gccjit" ; create_dir (gccjit_target) ? ; if args . build_sysroot { println ! ("[BUILD] sysroot") ; build_sysroot (& env , & args . config_info) ? ; } Ok (()) }
};
}
