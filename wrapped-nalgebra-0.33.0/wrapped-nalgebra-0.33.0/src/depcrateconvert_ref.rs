// Generated macro for convert_ref (function)
macro_rules! Depcrateconvert_ref {
() => {
// Module: crate
// Provides: {"convert_ref"}
// Dependencies: {}
# [doc = " Converts an object from one type to an equivalent or more general one."] # [doc = ""] # [doc = " # See also:"] # [doc = ""] # [doc = " * [`convert()`]"] # [doc = " * [`convert_ref_unchecked()`]"] # [doc = " * [`is_convertible()`]"] # [doc = " * [`try_convert()`]"] # [doc = " * [`try_convert_ref()`]"] # [inline] pub fn convert_ref < From , To : SupersetOf < From > > (t : & From) -> To { To :: from_subset (t) }
};
}
