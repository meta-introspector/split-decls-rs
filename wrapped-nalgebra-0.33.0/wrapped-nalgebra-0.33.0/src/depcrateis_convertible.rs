// Generated macro for is_convertible (function)
macro_rules! Depcrateis_convertible {
() => {
// Module: crate
// Provides: {"is_convertible"}
// Dependencies: {}
# [doc = " Indicates if [`try_convert()`] will succeed without"] # [doc = " actually performing the conversion."] # [doc = ""] # [doc = " # See also:"] # [doc = ""] # [doc = " * [`convert()`]"] # [doc = " * [`convert_ref()`]"] # [doc = " * [`convert_ref_unchecked()`]"] # [doc = " * [`try_convert()`]"] # [doc = " * [`try_convert_ref()`]"] # [inline] pub fn is_convertible < From : SupersetOf < To > , To > (t : & From) -> bool { t . is_in_subset () }
};
}
