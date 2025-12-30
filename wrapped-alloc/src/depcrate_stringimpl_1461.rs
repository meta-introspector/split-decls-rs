// Generated macro for impl_1461 (impl)
macro_rules! Depcrate_stringimpl_1461 {
() => {
// Module: crate::string
// Provides: {"impl_1461"}
// Dependencies: {}
# [doc = " Implements the `+=` operator for appending to a `String`."] # [doc = ""] # [doc = " This has the same behavior as the [`push_str`][String::push_str] method."] # [cfg (not (no_global_oom_handling))] # [stable (feature = "stringaddassign" , since = "1.12.0")] impl AddAssign < & str > for String { # [inline] fn add_assign (& mut self , other : & str) { self . push_str (other) ; } }
};
}
