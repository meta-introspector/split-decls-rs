macro_rules! system_prefix_from_exepath_var {
    () => {
        fn system_prefix_from_exepath_var < F > (var_os_func : F) -> Option < PathBuf > where F : Fn (& str) -> Option < OsString > , { let root = var_os_func ("EXEPATH") . map (PathBuf :: from) . filter (| r | r . is_absolute ()) ? ; let mut candidates = ["clangarm64" , "mingw64" , "mingw32"] . iter () . map (| component | root . join (component)) . filter (| candidate | candidate . is_dir ()) ; let path = candidates . next () ? ; match candidates . next () { Some (_) => None , None => Some (path) , } }
    };
}

system_prefix_from_exepath_var!();