// Generated macro for impl_17 (impl)
macro_rules! Depcrate_commandsimpl_17 {
() => {
// Module: crate::commands
// Provides: {"impl_17"}
// Dependencies: {}
impl MiriEnv { # [doc = " Prepares the environment: builds miri and cargo-miri and a sysroot."] # [doc = " Returns the location of the sysroot."] # [doc = ""] # [doc = " If the target is None the sysroot will be built for the host machine."] fn build_miri_sysroot (& mut self , quiet : bool , target : Option < impl AsRef < OsStr > > , features : & [String] ,) -> Result < PathBuf > { if let Some (miri_sysroot) = self . sh . var_os ("MIRI_SYSROOT") { return Ok (miri_sysroot . into ()) ; } self . build ("." , features , & [] , quiet) ? ; self . build ("cargo-miri" , & [] , & [] , quiet) ? ; let target_flag = if let Some (target) = & target { vec ! [OsStr :: new ("--target") , target . as_ref ()] } else { vec ! [] } ; let target_flag = & target_flag ; if ! quiet { eprint ! ("$ cargo miri setup") ; if let Some (target) = & target { eprint ! (" --target {target}" , target = target . as_ref () . to_string_lossy ()) ; } eprintln ! () ; } let mut cmd = self . cargo_cmd ("cargo-miri" , "run" , & []) . arg ("--quiet") . arg ("--") . args (& ["miri" , "setup" , "--print-sysroot"]) . args (target_flag) ; cmd . set_quiet (quiet) ; let output = cmd . read () ? ; self . sh . set_var ("MIRI_SYSROOT" , & output) ; Ok (output . into ()) } }
};
}
