// Generated macro for type_len_with_wrapped_val (function)
macro_rules! Depcrate_macros_reflecttype_len_with_wrapped_val {
() => {
// Module: crate::macros::reflect
// Provides: {"type_len_with_wrapped_val"}
// Dependencies: {}
# [doc = " Length __in bytes__ of the [`format_type!`] macro result."] # [must_use] pub const fn type_len_with_wrapped_val (ty : Type , val : WrappedValue) -> usize { let mut len = ty . len () + "!" . len () ; let mut curr = val ; while curr % 10 != 0 { match curr % 10 { 2 => len -= "!" . len () , 3 => len += "[]!" . len () , _ => { } } curr /= 10 ; } len }
};
}
