// Generated macro for SymlinkCheck (struct)
macro_rules! DepcrateSymlinkCheck {
() => {
// Module: crate
// Provides: {"SymlinkCheck"}
// Dependencies: {}
# [doc = " A stack that validates we are not going through a symlink in a way that is read-only."] # [doc = ""] # [doc = " It can efficiently validate paths when these are queried in sort-order, which leads to each component"] # [doc = " to only be checked once."] pub struct SymlinkCheck { # [doc = " Supports querying additional information, like the stack root."] pub inner : gix_fs :: Stack , }
};
}
