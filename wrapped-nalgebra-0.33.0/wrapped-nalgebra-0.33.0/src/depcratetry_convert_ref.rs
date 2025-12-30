// Generated macro for try_convert_ref (function)
macro_rules! Depcratetry_convert_ref {
() => {
// Module: crate
// Provides: {"try_convert_ref"}
// Dependencies: {}
# [doc = " Attempts to convert an object to a more specific one."] # [doc = ""] # [doc = " # See also:"] # [doc = ""] # [doc = " * [`convert()`]"] # [doc = " * [`convert_ref()`]"] # [doc = " * [`convert_ref_unchecked()`]"] # [doc = " * [`is_convertible()`]"] # [doc = " * [`try_convert()`]"] # [inline] pub fn try_convert_ref < From : SupersetOf < To > , To > (t : & From) -> Option < To > { t . to_subset () }
};
}
