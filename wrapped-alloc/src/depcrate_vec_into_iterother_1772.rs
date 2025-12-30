// Generated macro for other_1772 (other)
macro_rules! Depcrate_vec_into_iterother_1772 {
() => {
// Module: crate::vec::into_iter
// Provides: {"other_1772"}
// Dependencies: {}
macro non_null { (mut $ place : expr , $ t : ident) => { { #! [allow (unused_unsafe)] unsafe { & mut * ((& raw mut $ place) as * mut NonNull <$ t >) } } } , ($ place : expr , $ t : ident) => { { #! [allow (unused_unsafe)] unsafe { * ((& raw const $ place) as * const NonNull <$ t >) } } } , }
};
}
