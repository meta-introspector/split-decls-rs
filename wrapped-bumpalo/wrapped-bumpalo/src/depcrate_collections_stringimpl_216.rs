// Generated macro for impl_216 (impl)
macro_rules! Depcrate_collections_stringimpl_216 {
() => {
// Module: crate::collections::string
// Provides: {"impl_216"}
// Dependencies: {}
# [doc = " Implements the `+=` operator for appending to a `String<'bump>`."] # [doc = ""] # [doc = " This has the same behavior as the [`push_str`][String::push_str] method."] impl < 'a , 'bump > AddAssign < & 'a str > for String < 'bump > { # [inline] fn add_assign (& mut self , other : & str) { self . push_str (other) ; } }
};
}
