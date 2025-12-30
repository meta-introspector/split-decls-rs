// Generated macro for WalkParallel (struct)
macro_rules! Depcrate_walkWalkParallel {
() => {
// Module: crate::walk
// Provides: {"WalkParallel"}
// Dependencies: {}
# [doc = " WalkParallel is a parallel recursive directory iterator over files paths"] # [doc = " in one or more directories."] # [doc = ""] # [doc = " Only file and directory paths matching the rules are returned. By default,"] # [doc = " ignore files like `.gitignore` are respected. The precise matching rules"] # [doc = " and precedence is explained in the documentation for `WalkBuilder`."] # [doc = ""] # [doc = " Unlike `Walk`, this uses multiple threads for traversing a directory."] pub struct WalkParallel { paths : std :: vec :: IntoIter < PathBuf > , ig_root : Ignore , max_filesize : Option < u64 > , max_depth : Option < usize > , min_depth : Option < usize > , follow_links : bool , same_file_system : bool , threads : usize , skip : Option < Arc < Handle > > , filter : Option < Filter > , }
};
}
