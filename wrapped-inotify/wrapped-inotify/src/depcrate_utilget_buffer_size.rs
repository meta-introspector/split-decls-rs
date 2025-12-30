// Generated macro for get_buffer_size (function)
macro_rules! Depcrate_utilget_buffer_size {
() => {
// Module: crate::util
// Provides: {"get_buffer_size"}
// Dependencies: {}
# [doc = " Get the inotify event buffer size"] # [doc = ""] # [doc = " The maximum size of an inotify event and thus the buffer size to hold it"] # [doc = " can be calculated using this formula:"] # [doc = " `sizeof(struct inotify_event) + NAME_MAX + 1`"] # [doc = ""] # [doc = " See: <https://man7.org/linux/man-pages/man7/inotify.7.html>"] # [doc = ""] # [doc = " The NAME_MAX size formula is:"] # [doc = " `ABSOLUTE_PARENT_PATH_LEN + 1 + 255`"] # [doc = ""] # [doc = " - `ABSOLUTE_PARENT_PATH_LEN` will be calculated at runtime."] # [doc = " - Add 1 to account for a `/`, either in between the parent path and a filename or for the root directory."] # [doc = " - Add the maximum number of chars in a filename, 255."] # [doc = ""] # [doc = " See: <https://github.com/torvalds/linux/blob/master/include/uapi/linux/limits.h>"] # [doc = ""] # [doc = " Unfortunately, we can't just do the same with max path length itself."] # [doc = ""] # [doc = " See: <https://eklitzke.org/path-max-is-tricky>"] # [doc = ""] # [doc = " This function is really just a fallible wrapper around `get_absolute_path_buffer_size()`."] # [doc = ""] # [doc = " path: A relative or absolute path for the inotify events."] pub fn get_buffer_size (path : & Path) -> io :: Result < usize > { Ok (get_absolute_path_buffer_size (& path . canonicalize () ?)) }
};
}
