// Generated macro for resolve (function)
macro_rules! Depcrate_alternateresolve {
() => {
// Module: crate::alternate
// Provides: {"resolve"}
// Dependencies: {}
# [doc = " Given an `objects_directory`, try to resolve alternate object directories possibly located in the"] # [doc = " `./info/alternates` file into canonical paths and resolve relative paths with the help of the `current_dir`."] # [doc = " If no alternate object database was resolved, the resulting `Vec` is empty (it is not an error"] # [doc = " if there are no alternates)."] # [doc = " It is an error once a repository is seen again as it would lead to a cycle."] pub fn resolve (objects_directory : PathBuf , current_dir : & std :: path :: Path) -> Result < Vec < PathBuf > , Error > { let mut dirs = vec ! [(0 , objects_directory . clone ())] ; let mut out = Vec :: new () ; let mut seen = vec ! [gix_path :: realpath_opts (& objects_directory , current_dir , MAX_SYMLINKS) ?] ; while let Some ((depth , dir)) = dirs . pop () { match fs :: read (dir . join ("info") . join ("alternates")) { Ok (input) => { for path in parse :: content (& input) ? . into_iter () { let path = objects_directory . join (path) ; let path_canonicalized = gix_path :: realpath_opts (& path , current_dir , MAX_SYMLINKS) ? ; if seen . contains (& path_canonicalized) { return Err (Error :: Cycle (seen)) ; } seen . push (path_canonicalized) ; dirs . push ((depth + 1 , path)) ; } } Err (err) if err . kind () == io :: ErrorKind :: NotFound => { } Err (err) => return Err (err . into ()) , } if depth != 0 { out . push (dir) ; } } Ok (out) }
};
}
