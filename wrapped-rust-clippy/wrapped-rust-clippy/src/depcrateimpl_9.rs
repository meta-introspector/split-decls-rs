// Generated macro for impl_9 (impl)
macro_rules! Depcrateimpl_9 {
() => {
// Module: crate
// Provides: {"impl_9"}
// Dependencies: {}
impl ClippyCmd { fn new < I > (mut old_args : I) -> Self where I : Iterator < Item = String > , { let mut cargo_subcommand = "check" ; let mut args = vec ! [] ; let mut clippy_args : Vec < String > = vec ! [] ; for arg in old_args . by_ref () { match arg . as_str () { "--fix" => { cargo_subcommand = "fix" ; continue ; } , "--no-deps" => { clippy_args . push ("--no-deps" . into ()) ; continue ; } , "--" => break , _ => { } , } args . push (arg) ; } clippy_args . append (& mut (old_args . collect ())) ; if cargo_subcommand == "fix" && ! clippy_args . iter () . any (| arg | arg == "--no-deps") { clippy_args . push ("--no-deps" . into ()) ; } Self { cargo_subcommand , args , clippy_args , } } fn path () -> PathBuf { let mut path = env :: current_exe () . expect ("current executable path invalid") . with_file_name ("clippy-driver") ; if cfg ! (windows) { path . set_extension ("exe") ; } path } fn into_std_cmd (self) -> Command { let mut cmd = Command :: new (env :: var ("CARGO") . unwrap_or_else (| _ | "cargo" . into ())) ; let clippy_args : String = self . clippy_args . iter () . fold (String :: new () , | s , arg | s + arg + "__CLIPPY_HACKERY__") ; let terminal_width = termize :: dimensions () . map_or (0 , | (w , _) | w) ; cmd . env ("RUSTC_WORKSPACE_WRAPPER" , Self :: path ()) . env ("CLIPPY_ARGS" , clippy_args) . env ("CLIPPY_TERMINAL_WIDTH" , terminal_width . to_string ()) . arg (self . cargo_subcommand) . args (& self . args) ; cmd } }
};
}
