// Generated macro for type_eq_non_static (function)
macro_rules! Depcrate_utilstype_eq_non_static {
() => {
// Module: crate::utils
// Provides: {"type_eq_non_static"}
// Dependencies: {}
# [doc = " Determine if two generic types which may not be static are equal to each"] # [doc = " other."] # [doc = ""] # [doc = " This function must be used with extreme discretion, as no lifetime checking"] # [doc = " is done. Meaning, this function considers `Struct<'a>` to be equal to"] # [doc = " `Struct<'b>`, even if either `'a` or `'b` outlives the other."] # [inline (always)] pub (crate) fn type_eq_non_static < T : ? Sized , U : ? Sized > () -> bool { non_static_type_id :: < T > () == non_static_type_id :: < U > () }
};
}
