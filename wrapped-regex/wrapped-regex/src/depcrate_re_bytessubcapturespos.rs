// Generated macro for SubCapturesPos (struct)
macro_rules! Depcrate_re_bytesSubCapturesPos {
() => {
// Module: crate::re_bytes
// Provides: {"SubCapturesPos"}
// Dependencies: {}
# [doc = " An iterator over capture group positions for a particular match of a"] # [doc = " regular expression."] # [doc = ""] # [doc = " Positions are byte indices in terms of the original byte string matched."] # [doc = ""] # [doc = " `'c` is the lifetime of the captures."] pub struct SubCapturesPos < 'c > { idx : usize , slots : & 'c [Option < usize >] }
};
}
