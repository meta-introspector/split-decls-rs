// Generated macro for Iter (struct)
macro_rules! Depcrate_dir_removeIter {
() => {
// Module: crate::dir::remove
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " A special iterator which communicates its operation through results where…"] # [doc = ""] # [doc = " * `Some(Ok(removed_directory))` is yielded once or more success, followed by `None`"] # [doc = " * `Some(Err(std::io::Error))` is yielded exactly once on failure."] pub struct Iter < 'a > { cursor : Option < & 'a Path > , boundary : & 'a Path , }
};
}
