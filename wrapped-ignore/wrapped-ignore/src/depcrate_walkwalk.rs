// Generated macro for Walk (struct)
macro_rules! Depcrate_walkWalk {
() => {
// Module: crate::walk
// Provides: {"Walk"}
// Dependencies: {}
# [doc = " Walk is a recursive directory iterator over file paths in one or more"] # [doc = " directories."] # [doc = ""] # [doc = " Only file and directory paths matching the rules are returned. By default,"] # [doc = " ignore files like `.gitignore` are respected. The precise matching rules"] # [doc = " and precedence is explained in the documentation for `WalkBuilder`."] pub struct Walk { its : std :: vec :: IntoIter < (PathBuf , Option < WalkEventIter >) > , it : Option < WalkEventIter > , ig_root : Ignore , ig : Ignore , max_filesize : Option < u64 > , skip : Option < Arc < Handle > > , filter : Option < Filter > , }
};
}
