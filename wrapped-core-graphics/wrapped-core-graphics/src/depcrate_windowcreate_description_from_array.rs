// Generated macro for create_description_from_array (function)
macro_rules! Depcrate_windowcreate_description_from_array {
() => {
// Module: crate::window
// Provides: {"create_description_from_array"}
// Dependencies: {}
pub fn create_description_from_array (window_array : CFArray < CGWindowID > ,) -> Option < CFArray < CFDictionary < CFString , CFType > > > { unsafe { let array = CGWindowListCreateDescriptionFromArray (window_array . as_concrete_TypeRef ()) ; if array . is_null () { None } else { Some (TCFType :: wrap_under_create_rule (array)) } } }
};
}
