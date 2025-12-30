// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () , Box < dyn Error > > { let mut path : PathBuf = env :: args_os () . nth (1) . expect ("a path to the rust repository is required") . into () ; path . push ("library/std/src/sys/pal/windows/c") ; env :: set_current_dir (& path) ? ; sort_bindings ("bindings.txt") ? ; windows_bindgen :: bindgen (["--etc" , "bindings.txt"]) ; let mut f = std :: fs :: File :: options () . append (true) . open ("windows_sys.rs") ? ; f . write_all (ARM32_SHIM . as_bytes ()) ? ; writeln ! (& mut f , "// ignore-tidy-filelength") ? ; Ok (()) }
};
}
