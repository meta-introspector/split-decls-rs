// Generated macro for open_options_no_follow (function)
macro_rules! Depcrate_fsopen_options_no_follow {
() => {
// Module: crate::fs
// Provides: {"open_options_no_follow"}
// Dependencies: {}
# [doc = " Prepare open options which won't follow symlinks when the file is opened."] # [doc = ""] # [doc = " Note: only effective on unix currently."] pub fn open_options_no_follow () -> std :: fs :: OpenOptions { # [cfg_attr (not (unix) , allow (unused_mut))] let mut options = std :: fs :: OpenOptions :: new () ; # [cfg (unix)] { # [doc = " Make sure that it's impossible to follow through to the target of symlinks."] # [doc = " Note that this will still follow symlinks in the path, which is what we assume"] # [doc = " has been checked separately."] use std :: os :: unix :: fs :: OpenOptionsExt ; options . custom_flags (libc :: O_NOFOLLOW) ; } options }
};
}
