// Generated macro for check_enum_start (function)
macro_rules! Depcrate_item_name_repetitionscheck_enum_start {
() => {
// Module: crate::item_name_repetitions
// Provides: {"check_enum_start"}
// Dependencies: {}
fn check_enum_start (cx : & LateContext < '_ > , item_name : & str , variant : & Variant < '_ >) { let name = variant . ident . name . as_str () ; let item_name_chars = item_name . chars () . count () ; if count_match_start (item_name , name) . char_count == item_name_chars && name . chars () . nth (item_name_chars) . is_some_and (| c | ! c . is_lowercase ()) && name . chars () . nth (item_name_chars + 1) . is_some_and (| c | ! c . is_numeric ()) && ! check_enum_tuple_path_match (name , variant . data) { span_lint_hir (cx , ENUM_VARIANT_NAMES , variant . hir_id , variant . span , "variant name starts with the enum's name" ,) ; } }
};
}
