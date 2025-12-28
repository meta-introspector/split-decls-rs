macro_rules! dylib_path {
    () => {
        # [doc = " Returns a list of directories that are searched for dynamic libraries."] # [doc = ""] # [doc = " Note that some operating systems will have defaults if this is empty that"] # [doc = " will need to be dealt with."] pub fn dylib_path () -> Vec < PathBuf > { match env :: var_os (dylib_path_envvar ()) { Some (var) => env :: split_paths (& var) . collect () , None => Vec :: new () , } }
    };
}

dylib_path!()