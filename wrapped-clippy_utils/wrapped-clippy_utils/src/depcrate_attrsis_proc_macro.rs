// Generated macro for is_proc_macro (function)
macro_rules! Depcrate_attrsis_proc_macro {
() => {
// Module: crate::attrs
// Provides: {"is_proc_macro"}
// Dependencies: {}
# [doc = " Checks whether `attrs` contain any of `proc_macro`, `proc_macro_derive` or"] # [doc = " `proc_macro_attribute`"] pub fn is_proc_macro (attrs : & [impl AttributeExt]) -> bool { attrs . iter () . any (AttributeExt :: is_proc_macro_attr) }
};
}
