// Generated macro for Size (enum)
macro_rules! Depcrate_fs_fieldsSize {
() => {
// Module: crate::fs::fields
// Provides: {"Size"}
// Dependencies: {}
# [doc = " A file’s size, in bytes. This is usually formatted by the `number_prefix`"] # [doc = " crate into something human-readable."] # [derive (Copy , Clone)] pub enum Size { # [doc = " This file has a defined size."] Some (u64) , # [doc = " This file has no size, or has a size but we aren’t interested in it."] # [doc = ""] # [doc = " Under Unix, directory entries that aren’t regular files will still"] # [doc = " have a file size. For example, a directory will just contain a list of"] # [doc = " its files as its “contents” and will be specially flagged as being a"] # [doc = " directory, rather than a file. However, seeing the “file size” of this"] # [doc = " data is rarely useful — I can’t think of a time when I’ve seen it and"] # [doc = " learnt something. So we discard it and just output “-” instead."] # [doc = ""] # [doc = " See this answer for more: <https://unix.stackexchange.com/a/68266>"] None , # [doc = " This file is a block or character device, so instead of a size, print"] # [doc = " out the file’s major and minor device IDs."] # [doc = ""] # [doc = " This is what ls does as well. Without it, the devices will just have"] # [doc = " file sizes of zero."] DeviceIDs (DeviceIDs) , }
};
}
