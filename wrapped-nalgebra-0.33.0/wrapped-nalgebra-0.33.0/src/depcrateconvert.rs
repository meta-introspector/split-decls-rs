// Generated macro for convert (function)
macro_rules! Depcrateconvert {
() => {
// Module: crate
// Provides: {"convert"}
// Dependencies: {}
# [doc = " Converts an object from one type to an equivalent or more general one."] # [doc = ""] # [doc = " See also [`try_convert()`] for conversion to more specific types."] # [doc = ""] # [doc = " # See also:"] # [doc = ""] # [doc = " * [`convert_ref()`]"] # [doc = " * [`convert_ref_unchecked()`]"] # [doc = " * [`is_convertible()`]"] # [doc = " * [`try_convert()`]"] # [doc = " * [`try_convert_ref()`]"] # [inline] pub fn convert < From , To : SupersetOf < From > > (t : From) -> To { To :: from_subset (& t) }
};
}
