// Generated macro for write_delimited (function)
macro_rules! Depcrate_error_utilwrite_delimited {
() => {
// Module: crate::error::util
// Provides: {"write_delimited"}
// Dependencies: {}
# [doc = " Iterate through a list of items, writing each of them to the target separated"] # [doc = " by a delimiter string."] pub (crate) fn write_delimited < T : fmt :: Display > (f : & mut impl fmt :: Write , items : impl IntoIterator < Item = T > , delimiter : & str ,) -> fmt :: Result { let mut first = true ; for item in items { if ! first { write ! (f , "{}" , delimiter) ? ; } first = false ; write ! (f , "{}" , item) ? ; } Ok (()) }
};
}
