// Generated macro for is_strategy (function)
macro_rules! Depcrate_property_test_utilsis_strategy {
() => {
// Module: crate::property_test::utils
// Provides: {"is_strategy"}
// Dependencies: {}
# [doc = " Checks if an attribute counts as a \"strategy\" attribute"] # [doc = ""] # [doc = " This means:"] # [doc = "  - it is an outer attribute (i.e. `#[...]` not `#![...]`)"] # [doc = "  - it contains `strategy = <expr>`"] pub fn is_strategy (attr : & Attribute) -> bool { let path_correct = attr . path () . get_ident () . map (| ident | ident == "strategy") . unwrap_or (false) ; let has_equals = matches ! (& attr . meta , Meta :: NameValue (_)) ; let is_outer = matches ! (attr . style , AttrStyle :: Outer) ; path_correct && has_equals && is_outer }
};
}
