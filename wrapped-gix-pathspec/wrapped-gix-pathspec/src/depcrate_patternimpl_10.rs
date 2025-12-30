// Generated macro for impl_10 (impl)
macro_rules! Depcrate_patternimpl_10 {
() => {
// Module: crate::pattern
// Provides: {"impl_10"}
// Dependencies: {}
# [doc = " Access"] impl Pattern { # [doc = " Returns `true` if this seems to be a pathspec that indicates that 'there is no pathspec'."] # [doc = ""] # [doc = " Note that such a spec is `:`."] pub fn is_nil (& self) -> bool { self . nil } # [doc = " Return the prefix-portion of the `path` of this spec, which is a *directory*."] # [doc = " It can be empty if there is no prefix."] # [doc = ""] # [doc = " A prefix is effectively the CWD seen as relative to the working tree, and it's assumed to"] # [doc = " match case-sensitively. This makes it useful for skipping over large portions of input by"] # [doc = " directly comparing them."] pub fn prefix_directory (& self) -> & BStr { self . path [.. self . prefix_len] . as_bstr () } # [doc = " Return the path of this spec, typically used for matching."] pub fn path (& self) -> & BStr { self . path . as_ref () } }
};
}
