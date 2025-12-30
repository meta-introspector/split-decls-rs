// Generated macro for impl_37 (impl)
macro_rules! Depcrate_clone_gccimpl_37 {
() => {
// Module: crate::clone_gcc
// Provides: {"impl_37"}
// Dependencies: {}
impl Args { fn new () -> Result < Option < Self > , String > { let mut command_args = Self :: default () ; let mut out_path = None ; let mut args = std :: env :: args () . skip (2) ; while let Some (arg) = args . next () { match arg . as_str () { "--out-path" => match args . next () { Some (path) if ! path . is_empty () => out_path = Some (path) , _ => { return Err ("Expected an argument after `--out-path`, found nothing" . into ()) ; } } , "--help" => { show_usage () ; return Ok (None) ; } arg => { if ! command_args . config_info . parse_argument (arg , & mut args) ? { return Err (format ! ("Unknown option {arg}")) ; } } } } command_args . out_path = match out_path { Some (p) => p . into () , None => PathBuf :: from ("./gcc") , } ; Ok (Some (command_args)) } }
};
}
