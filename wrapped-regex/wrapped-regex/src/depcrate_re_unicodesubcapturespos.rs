// Generated macro for SubCapturesPos (struct)
macro_rules! Depcrate_re_unicodeSubCapturesPos {
() => {
// Module: crate::re_unicode
// Provides: {"SubCapturesPos"}
// Dependencies: {}
# [doc = " An iterator over capture group positions for a particular match of a"] # [doc = " regular expression."] # [doc = ""] # [doc = " Positions are byte indices in terms of the original string matched."] # [doc = ""] # [doc = " `'c` is the lifetime of the captures."] pub struct SubCapturesPos < 'c > { idx : usize , slots : & 'c [Option < usize >] }
};
}
