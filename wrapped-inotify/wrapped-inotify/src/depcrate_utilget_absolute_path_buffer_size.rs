// Generated macro for get_absolute_path_buffer_size (function)
macro_rules! Depcrate_utilget_absolute_path_buffer_size {
() => {
// Module: crate::util
// Provides: {"get_absolute_path_buffer_size"}
// Dependencies: {}
# [doc = " Get the inotify event buffer size for an absolute path"] # [doc = ""] # [doc = " For relative paths, consider using `get_buffer_size()` which provides a fallible wrapper"] # [doc = " for this function."] # [doc = ""] # [doc = " path: An absolute path for the inotify events."] pub fn get_absolute_path_buffer_size (path : & Path) -> usize { let parent_path_len = path . parent () . map (| parent_path | parent_path . as_os_str () . len ()) . unwrap_or (0) ; INOTIFY_EVENT_SIZE + parent_path_len }
};
}
