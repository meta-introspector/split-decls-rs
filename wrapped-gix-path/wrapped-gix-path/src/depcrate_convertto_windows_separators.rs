// Generated macro for to_windows_separators (function)
macro_rules! Depcrate_convertto_windows_separators {
() => {
// Module: crate::convert
// Provides: {"to_windows_separators"}
// Dependencies: {}
# [doc = " Find slashes and replace them with backslashes, unconditionally."] # [doc = ""] # [doc = " **Note** Do not use these and prefer the conditional versions of this method."] pub fn to_windows_separators < 'a > (path : impl Into < Cow < 'a , BStr > >) -> Cow < 'a , BStr > { replace (path , b'/' , b'\\') }
};
}
