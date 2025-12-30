// Generated macro for FixedArray (trait)
macro_rules! Depcrate_arg_msgargFixedArray {
() => {
// Module: crate::arg::msgarg
// Provides: {"FixedArray"}
// Dependencies: {}
# [doc = " If a type implements this trait, it means the size and alignment is the same"] # [doc = " as in D-Bus. This means that you can quickly append and get slices of this type."] # [doc = ""] # [doc = " Note: Booleans do not implement this trait because D-Bus booleans are 4 bytes and Rust booleans are 1 byte."] pub unsafe trait FixedArray : Arg + 'static + Clone + Copy { }
};
}
