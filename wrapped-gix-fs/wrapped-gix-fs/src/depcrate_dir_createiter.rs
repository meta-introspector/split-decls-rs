// Generated macro for Iter (struct)
macro_rules! Depcrate_dir_createIter {
() => {
// Module: crate::dir::create
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " A special iterator which communicates its operation through results where…"] # [doc = ""] # [doc = " * `Some(Ok(created_directory))` is yielded once or more success, followed by `None`"] # [doc = " * `Some(Err(Error::Intermediate))` is yielded zero or more times while trying to create the directory."] # [doc = " * `Some(Err(Error::Permanent))` is yielded exactly once on failure."] pub struct Iter < 'a > { cursors : Vec < & 'a Path > , retries : Retries , original_retries : Retries , state : State , }
};
}
