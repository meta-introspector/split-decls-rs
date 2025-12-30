// Generated macro for empty_upward_until_boundary (function)
macro_rules! Depcrate_dir_removeempty_upward_until_boundary {
() => {
// Module: crate::dir::remove
// Provides: {"empty_upward_until_boundary"}
// Dependencies: {}
# [doc = " Delete all empty directories from `delete_dir` upward and until (not including) the `boundary_dir`."] # [doc = ""] # [doc = " Note that `boundary_dir` must contain `delete_dir` or an error is returned, otherwise `delete_dir` is returned on success."] pub fn empty_upward_until_boundary < 'a > (delete_dir : & 'a Path , boundary_dir : & 'a Path) -> std :: io :: Result < & 'a Path > { for item in Iter :: new (delete_dir , boundary_dir) ? { match item { Ok (_dir) => continue , Err (err) => return Err (err) , } } Ok (delete_dir) }
};
}
