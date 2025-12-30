// Generated macro for to_unix_separators (function)
macro_rules! Depcrate_convertto_unix_separators {
() => {
// Module: crate::convert
// Provides: {"to_unix_separators"}
// Dependencies: {}
# [doc = " Replace Windows path separators with slashes, which typically resembles a Unix path, unconditionally."] # [doc = ""] # [doc = " **Note** Do not use these and prefer the conditional versions of this method."] pub fn to_unix_separators < 'a > (path : impl Into < Cow < 'a , BStr > >) -> Cow < 'a , BStr > { replace (path , b'\\' , b'/') }
};
}
