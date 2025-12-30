// Generated macro for category_and_type_from_name (function)
macro_rules! Depcratecategory_and_type_from_name {
() => {
// Module: crate
// Provides: {"category_and_type_from_name"}
// Dependencies: {}
pub fn category_and_type_from_name (name : & str) -> (String , String) { let mut category = "" . to_string () ; let mut ty = "" . to_string () ; let split : Vec < & str > = name . split (':') . collect () ; if let Some (cat) = split . first () { category = cat . to_string () ; } if let Some (t) = split . get (1) { ty = t . to_string () ; } (category , ty) }
};
}
