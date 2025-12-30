// Generated macro for is_enum_variant_ctor (function)
macro_rules! Depcrateis_enum_variant_ctor {
() => {
// Module: crate
// Provides: {"is_enum_variant_ctor"}
// Dependencies: {}
# [doc = " Checks if `{ctor_call_id}(...)` is `{enum_item}::{variant_name}(...)`."] pub fn is_enum_variant_ctor (cx : & LateContext < '_ > , enum_item : Symbol , variant_name : Symbol , ctor_call_id : DefId ,) -> bool { let Some (enum_def_id) = cx . tcx . get_diagnostic_item (enum_item) else { return false ; } ; let variants = cx . tcx . adt_def (enum_def_id) . variants () . iter () ; variants . filter (| variant | variant . name == variant_name) . filter_map (| variant | variant . ctor . as_ref ()) . any (| (_ , ctor_def_id) | * ctor_def_id == ctor_call_id) }
};
}
