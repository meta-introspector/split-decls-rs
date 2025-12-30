// Generated macro for add_rustdoc_cargo_linker_args (function)
macro_rules! Depcrate_utils_helpersadd_rustdoc_cargo_linker_args {
() => {
// Module: crate::utils::helpers
// Provides: {"add_rustdoc_cargo_linker_args"}
// Dependencies: {}
pub fn add_rustdoc_cargo_linker_args (cmd : & mut BootstrapCommand , builder : & Builder < '_ > , target : TargetSelection , lld_threads : LldThreads ,) { let args = linker_args (builder , target , lld_threads) ; let mut flags = cmd . get_envs () . find_map (| (k , v) | if k == OsStr :: new ("RUSTDOCFLAGS") { v } else { None }) . unwrap_or_default () . to_os_string () ; for arg in args { if ! flags . is_empty () { flags . push (" ") ; } flags . push (arg) ; } if ! flags . is_empty () { cmd . env ("RUSTDOCFLAGS" , flags) ; } }
};
}
