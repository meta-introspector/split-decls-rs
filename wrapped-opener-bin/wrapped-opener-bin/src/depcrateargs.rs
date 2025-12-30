// Generated macro for Args (struct)
macro_rules! DepcrateArgs {
() => {
// Module: crate
// Provides: {"Args"}
// Dependencies: {}
# [derive (Debug , StructOpt)] struct Args { # [doc = " The path to open"] # [structopt (parse (from_os_str))] path : PathBuf , # [doc = " Open the path with the `open_browser()` function"] # [structopt (long = "browser")] browser : bool , # [doc = " Reveal the file in the file explorer instead of opening it"] # [structopt (long = "reveal" , short = "R" , conflicts_with = "browser")] reveal : bool , }
};
}
