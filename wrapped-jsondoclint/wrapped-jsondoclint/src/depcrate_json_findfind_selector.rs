// Generated macro for find_selector (function)
macro_rules! Depcrate_json_findfind_selector {
() => {
// Module: crate::json_find
// Provides: {"find_selector"}
// Dependencies: {}
pub fn find_selector (haystack : & Value , needle : & Value) -> Vec < Selector > { let mut result = Vec :: new () ; let mut sel = Selector :: new () ; find_selector_recursive (haystack , needle , & mut result , & mut sel) ; result }
};
}
