// Generated macro for Matcher (struct)
macro_rules! Depcrate_globMatcher {
() => {
// Module: crate::glob
// Provides: {"Matcher"}
// Dependencies: {}
# [doc = " A matcher that can be iterated over for matched relative path buffers."] pub struct Matcher < 'a > { root : & 'a Root , queue : VecDeque < (RelativePathBuf , & 'a [Component < 'a >]) > , }
};
}
