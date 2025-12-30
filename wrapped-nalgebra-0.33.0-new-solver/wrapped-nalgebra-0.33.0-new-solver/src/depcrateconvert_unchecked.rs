// Generated macro for convert_unchecked (function)
macro_rules! Depcrateconvert_unchecked {
() => {
// Module: crate
// Provides: {"convert_unchecked"}
// Dependencies: {}
# [doc = " Use with care! Same as [`try_convert()`] but"] # [doc = " without any property checks."] # [doc = ""] # [doc = " # See also:"] # [doc = ""] # [doc = " * [`convert()`]"] # [doc = " * [`convert_ref()`]"] # [doc = " * [`convert_ref_unchecked()`]"] # [doc = " * [`is_convertible()`]"] # [doc = " * [`try_convert()`]"] # [doc = " * [`try_convert_ref()`]"] # [inline] pub fn convert_unchecked < From : SupersetOf < To > , To > (t : From) -> To { t . to_subset_unchecked () }
};
}
