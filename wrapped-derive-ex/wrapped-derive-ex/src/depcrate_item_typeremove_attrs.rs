// Generated macro for remove_attrs (function)
macro_rules! Depcrate_item_typeremove_attrs {
() => {
// Module: crate::item_type
// Provides: {"remove_attrs"}
// Dependencies: {}
fn remove_attrs (attrs : & mut Vec < Attribute > , kinds : & HelperAttributeKinds) { attrs . retain (| attr | ! kinds . is_match (attr)) ; }
};
}
