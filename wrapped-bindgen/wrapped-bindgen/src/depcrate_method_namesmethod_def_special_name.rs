// Generated macro for method_def_special_name (function)
macro_rules! Depcrate_method_namesmethod_def_special_name {
() => {
// Module: crate::method_names
// Provides: {"method_def_special_name"}
// Dependencies: {}
fn method_def_special_name (row : MethodDef) -> String { let name = row . name () ; if row . flags () . contains (MethodAttributes :: SpecialName) { if name . starts_with ("get") { name [4 ..] . to_string () } else if name . starts_with ("put") { format ! ("Set{}" , & name [4 ..]) } else if name . starts_with ("add") { name [4 ..] . to_string () } else if name . starts_with ("remove") { format ! ("Remove{}" , & name [7 ..]) } else { name . to_string () } } else { if let Some (attribute) = row . find_attribute ("OverloadAttribute") { for (_ , arg) in attribute . args () { if let Value :: Str (overload) = arg { if let Some (suffix) = overload . strip_prefix (name) { if suffix . parse :: < u32 > () . is_ok () { return name . to_string () ; } } return overload . to_string () ; } } } name . to_string () } }
};
}
