// Generated macro for impl_27 (impl)
macro_rules! Depcrate_fs_dirimpl_27 {
() => {
// Module: crate::fs::dir
// Provides: {"impl_27"}
// Dependencies: {}
impl Dir { # [doc = " Create a new Dir object filled with all the files in the directory"] # [doc = " pointed to by the given path. Fails if the directory can’t be read, or"] # [doc = " isn’t actually a directory, or if there’s an IO error that occurs at"] # [doc = " any point."] # [doc = ""] # [doc = " The `read_dir` iterator doesn’t actually yield the `.` and `..`"] # [doc = " entries, so if the user wants to see them, we’ll have to add them"] # [doc = " ourselves after the files have been read."] pub fn read_dir (path : PathBuf) -> io :: Result < Self > { info ! ("Reading directory {:?}" , & path) ; let contents = fs :: read_dir (& path) ? . collect :: < Result < Vec < _ > , _ > > () ? ; info ! ("Read directory success {:?}" , & path) ; Ok (Self { contents , path }) } # [doc = " Produce an iterator of IO results of trying to read all the files in"] # [doc = " this directory."] pub fn files < 'dir , 'ig > (& 'dir self , dots : DotFilter , git : Option < & 'ig GitCache > , git_ignoring : bool , deref_links : bool , total_size : bool ,) -> Files < 'dir , 'ig > { Files { inner : self . contents . iter () , dir : self , dotfiles : dots . shows_dotfiles () , dots : dots . dots () , git , git_ignoring , deref_links , total_size , } } # [doc = " Whether this directory contains a file with the given path."] pub fn contains (& self , path : & Path) -> bool { self . contents . iter () . any (| p | p . path () . as_path () == path) } # [doc = " Append a path onto the path specified by this directory."] pub fn join (& self , child : & Path) -> PathBuf { self . path . join (child) } }
};
}
