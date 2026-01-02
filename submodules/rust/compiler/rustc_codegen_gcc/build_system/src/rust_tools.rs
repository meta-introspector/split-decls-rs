mkuse!{use std :: collections :: HashMap ;}
mkuse!{use std :: ffi :: OsStr ;}
mkuse!{# [cfg (unix)] use std :: os :: unix :: process :: CommandExt ;}
mkuse!{use std :: path :: PathBuf ;}
mkuse!{use crate :: config :: ConfigInfo ;}
mkuse!{use crate :: utils :: { get_toolchain , rustc_toolchain_version_info , rustc_version_info } ;}

macro_rules! args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function args in module {}", module_path!());
    };
}

mkfn!{
    args_introspect!();
    fn args (command : & str) -> Result < Option < Vec < String > > , String > { if let Some ("--help") = std :: env :: args () . nth (2) . as_deref () { usage (command) ; return Ok (None) ; } let args = std :: env :: args () . skip (2) . collect :: < Vec < _ > > () ; if args . is_empty () { return Err (format ! ("Expected at least one argument for `{command}` subcommand, found none")) ; } Ok (Some (args)) }
}

macro_rules! usage_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function usage in module {}", module_path!());
    };
}

mkfn!{
    usage_introspect!();
    fn usage (command : & str) { println ! (r#"
`{command}` command help:

    [args]     : Arguments to be passed to the cargo command
    --help     : Show this help
"# ,) }
}
mkitem!{mkstruct!{struct RustcTools { env : HashMap < String , String > , args : Vec < String > , toolchain : String , config : ConfigInfo , }}}
mkitem!{mkimpl!{impl RustcTools { fn new (command : & str) -> Result < Option < Self > , String > { let Some (args) = args (command) ? else { return Ok (None) } ; let current_dir = std :: env :: current_dir () . and_then (| path | path . canonicalize ()) . map_err (| error | format ! ("Failed to get current directory path: {error:?}")) ? ; let current_exe = std :: env :: current_exe () . and_then (| path | path . canonicalize ()) . map_err (| error | format ! ("Failed to get current exe path: {error:?}")) ? ; let mut parent_dir = current_exe . components () . map (| comp | comp . as_os_str ()) . collect :: < Vec < _ > > () ; for to_remove in & ["y" , "release" , "target" , "build_system"] { if parent_dir . last () . map (| part | part == to_remove) . unwrap_or (false) { parent_dir . pop () ; } else { return Err (format ! ("Build script not executed from `build_system/target/release/y` (in path {})" , current_exe . display () ,)) ; } } let parent_dir = PathBuf :: from (parent_dir . join (OsStr :: new ("/"))) ; std :: env :: set_current_dir (& parent_dir) . map_err (| error | { format ! ("Failed to go to `{}` folder: {:?}" , parent_dir . display () , error) }) ? ; let mut env : HashMap < String , String > = std :: env :: vars () . collect () ; let mut config = ConfigInfo :: default () ; config . setup (& mut env , false) ? ; let toolchain = get_toolchain () ? ; let toolchain_version = rustc_toolchain_version_info (& toolchain) ? ; let default_version = rustc_version_info (None) ? ; if toolchain_version != default_version { println ! ("rustc_codegen_gcc is built for {} but the default rustc version is {}." , toolchain_version . short , default_version . short ,) ; println ! ("Using {}." , toolchain_version . short) ; } std :: env :: set_current_dir (& current_dir) . map_err (| error | { format ! ("Failed to go back to `{}` folder: {:?}" , current_dir . display () , error) }) ? ; let toolchain = format ! ("+{toolchain}") ; Ok (Some (Self { toolchain , args , env , config })) } }}}

macro_rules! exec_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exec in module {}", module_path!());
    };
}

mkfn!{
    exec_introspect!();
    fn exec (input : & [& dyn AsRef < OsStr >] , env : & HashMap < String , String >) -> Result < () , String > { # [cfg (unix)] { let error = crate :: utils :: get_command_inner (input , None , Some (env)) . exec () ; eprintln ! ("execvp syscall failed: {error:?}") ; std :: process :: exit (1) ; } # [cfg (not (unix))] { if crate :: utils :: run_command_with_output_and_env_no_err (input , None , Some (env)) . is_err () { std :: process :: exit (1) ; } Ok (()) } }
}

macro_rules! run_cargo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_cargo in module {}", module_path!());
    };
}

mkfn!{
    run_cargo_introspect!();
    pub fn run_cargo () -> Result < () , String > { let Some (mut tools) = RustcTools :: new ("cargo") ? else { return Ok (()) } ; let rustflags = tools . env . get ("RUSTFLAGS") . cloned () . unwrap_or_default () ; tools . env . insert ("RUSTDOCFLAGS" . to_string () , rustflags) ; let mut command : Vec < & dyn AsRef < OsStr > > = vec ! [& "cargo" , & tools . toolchain] ; for arg in & tools . args { command . push (arg) ; } exec (& command , & tools . env) }
}

macro_rules! run_rustc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run_rustc in module {}", module_path!());
    };
}

mkfn!{
    run_rustc_introspect!();
    pub fn run_rustc () -> Result < () , String > { let Some (tools) = RustcTools :: new ("rustc") ? else { return Ok (()) } ; let mut command = tools . config . rustc_command_vec () ; for arg in & tools . args { command . push (arg) ; } exec (& command , & tools . env) }
}