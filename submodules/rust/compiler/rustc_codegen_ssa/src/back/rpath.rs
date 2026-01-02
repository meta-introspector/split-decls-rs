mkuse!{use std :: ffi :: OsString ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use pathdiff :: diff_paths ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_fs_util :: try_canonicalize ;}
mkuse!{use tracing :: debug ;}
mkitem!{mkstruct!{pub (super) struct RPathConfig < 'a > { pub libs : & 'a [& 'a Path] , pub out_filename : PathBuf , pub is_like_darwin : bool , pub linker_is_gnu : bool , }}}

macro_rules! get_rpath_linker_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_rpath_linker_args in module {}", module_path!());
    };
}

mkfn!{
    get_rpath_linker_args_introspect!();
    pub (super) fn get_rpath_linker_args (config : & RPathConfig < '_ >) -> Vec < OsString > { debug ! ("preparing the RPATH!") ; let rpaths = get_rpaths (config) ; let mut args = Vec :: with_capacity (rpaths . len () * 2) ; for rpath in rpaths { args . push ("-rpath" . into ()) ; args . push (rpath) ; } if config . linker_is_gnu { args . push ("--enable-new-dtags" . into ()) ; args . push ("-z" . into ()) ; args . push ("origin" . into ()) ; } args }
}

macro_rules! get_rpaths_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_rpaths in module {}", module_path!());
    };
}

mkfn!{
    get_rpaths_introspect!();
    fn get_rpaths (config : & RPathConfig < '_ >) -> Vec < OsString > { debug ! ("output: {:?}" , config . out_filename . display ()) ; debug ! ("libs:") ; for libpath in config . libs { debug ! ("    {:?}" , libpath . display ()) ; } let rpaths = get_rpaths_relative_to_output (config) ; debug ! ("rpaths:") ; for rpath in & rpaths { debug ! ("    {:?}" , rpath) ; } minimize_rpaths (& rpaths) }
}

macro_rules! get_rpaths_relative_to_output_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_rpaths_relative_to_output in module {}", module_path!());
    };
}

mkfn!{
    get_rpaths_relative_to_output_introspect!();
    fn get_rpaths_relative_to_output (config : & RPathConfig < '_ >) -> Vec < OsString > { config . libs . iter () . map (| a | get_rpath_relative_to_output (config , a)) . collect () }
}

macro_rules! get_rpath_relative_to_output_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_rpath_relative_to_output in module {}", module_path!());
    };
}

mkfn!{
    get_rpath_relative_to_output_introspect!();
    fn get_rpath_relative_to_output (config : & RPathConfig < '_ > , lib : & Path) -> OsString { let prefix = if config . is_like_darwin { "@loader_path" } else { "$ORIGIN" } ; let lib = lib . parent () . unwrap () ; let output = config . out_filename . parent () . unwrap () ; let lib = if lib == Path :: new ("") { Path :: new (".") } else { lib } ; let output = if output == Path :: new ("") { Path :: new (".") } else { output } ; let lib = try_canonicalize (lib) . unwrap () ; let output = try_canonicalize (output) . unwrap () ; let relative = path_relative_from (& lib , & output) . unwrap_or_else (| | panic ! ("couldn't create relative path from {output:?} to {lib:?}")) ; let mut rpath = OsString :: from (prefix) ; rpath . push ("/") ; rpath . push (relative) ; rpath }
}

macro_rules! path_relative_from_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function path_relative_from in module {}", module_path!());
    };
}

mkfn!{
    path_relative_from_introspect!();
    fn path_relative_from (path : & Path , base : & Path) -> Option < PathBuf > { diff_paths (path , base) }
}

macro_rules! minimize_rpaths_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function minimize_rpaths in module {}", module_path!());
    };
}

mkfn!{
    minimize_rpaths_introspect!();
    fn minimize_rpaths (rpaths : & [OsString]) -> Vec < OsString > { let mut set = FxHashSet :: default () ; let mut minimized = Vec :: new () ; for rpath in rpaths { if set . insert (rpath) { minimized . push (rpath . clone ()) ; } } minimized }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}