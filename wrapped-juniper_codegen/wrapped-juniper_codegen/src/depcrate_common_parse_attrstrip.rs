// Generated macro for strip (function)
macro_rules! Depcrate_common_parse_attrstrip {
() => {
// Module: crate::common::parse::attr
// Provides: {"strip"}
// Dependencies: {}
# [doc = " Strips all `attr_path` attributes from the given `attrs` collection."] # [doc = ""] # [doc = " This function is generally used for removing duplicate attributes during `proc_macro_attribute`"] # [doc = " expansion, so avoid unnecessary expansion duplication."] pub (crate) fn strip (names : impl AttrNames , attrs : Vec < syn :: Attribute >) -> Vec < syn :: Attribute > { attrs . into_iter () . filter (| attr | ! path_eq_single (attr . path () , names)) . collect () }
};
}
