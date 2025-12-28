macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! path_to_repo_path {
    () => {
        deps!();
        # [doc = " Converts a path to a CString that is usable by the libgit2 API."] # [doc = ""] # [doc = " Checks if it is a relative path."] # [doc = ""] # [doc = " On Windows, this also requires the path to be valid Unicode, and translates"] # [doc = " back slashes to forward slashes."] pub fn path_to_repo_path (path : & Path) -> Result < CString , Error > { macro_rules ! err { ($ msg : literal , $ path : expr) => { return Err (Error :: from_str (& format ! ($ msg , $ path . display ()))) } ; } match path . components () . next () { None => return Err (Error :: from_str ("repo path should not be empty")) , Some (Component :: Prefix (_)) => err ! ("repo path `{}` should be relative, not a windows prefix" , path) , Some (Component :: RootDir) => err ! ("repo path `{}` should be relative" , path) , Some (Component :: CurDir) => err ! ("repo path `{}` should not start with `.`" , path) , Some (Component :: ParentDir) => err ! ("repo path `{}` should not start with `..`" , path) , Some (Component :: Normal (_)) => { } } # [cfg (windows)] { match path . to_str () { None => { return Err (Error :: from_str ("only valid unicode paths are accepted on windows" ,)) } Some (s) => return fixup_windows_path (s) , } } # [cfg (not (windows))] { path . into_c_string () } }
    };
}

path_to_repo_path!()