// Generated macro for try_convert (function)
macro_rules! Depcratetry_convert {
() => {
// Module: crate
// Provides: {"try_convert"}
// Dependencies: {}
# [doc = " Attempts to convert an object to a more specific one."] # [doc = ""] # [doc = " See also [`convert()`] for conversion to more general types."] # [doc = ""] # [doc = " # See also:"] # [doc = ""] # [doc = " * [`convert()`]"] # [doc = " * [`convert_ref()`]"] # [doc = " * [`convert_ref_unchecked()`]"] # [doc = " * [`is_convertible()`]"] # [doc = " * [`try_convert_ref()`]"] # [inline] pub fn try_convert < From : SupersetOf < To > , To > (t : From) -> Option < To > { t . to_subset () }
};
}
