// Generated macro for create_gitignore (function)
macro_rules! Depcrate_dircreate_gitignore {
() => {
// Module: crate::dir
// Provides: {"create_gitignore"}
// Dependencies: {}
# [doc = " Creates a new gitignore matcher for the directory given."] # [doc = ""] # [doc = " The matcher is meant to match files below `dir`."] # [doc = " Ignore globs are extracted from each of the file names relative to"] # [doc = " `dir_for_ignorefile` in the order given (earlier names have lower"] # [doc = " precedence than later names)."] # [doc = ""] # [doc = " I/O errors are ignored."] pub (crate) fn create_gitignore < T : AsRef < OsStr > > (dir : & Path , dir_for_ignorefile : & Path , names : & [T] , case_insensitive : bool ,) -> (Gitignore , Option < Error >) { let mut builder = GitignoreBuilder :: new (dir) ; let mut errs = PartialErrorBuilder :: default () ; builder . case_insensitive (case_insensitive) . unwrap () ; for name in names { let gipath = dir_for_ignorefile . join (name . as_ref ()) ; if cfg ! (windows) || gipath . exists () { errs . maybe_push_ignore_io (builder . add (gipath)) ; } } let gi = match builder . build () { Ok (gi) => gi , Err (err) => { errs . push (err) ; GitignoreBuilder :: new (dir) . build () . unwrap () } } ; (gi , errs . into_error_option ()) }
};
}
