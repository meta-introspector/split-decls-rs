// Generated macro for fn_is_externally_exported (function)
macro_rules! Depcrate_missing_inlinefn_is_externally_exported {
() => {
// Module: crate::missing_inline
// Provides: {"fn_is_externally_exported"}
// Dependencies: {}
# [doc = " Checks if this function is externally exported, where #[inline] wouldn't have the desired effect"] # [doc = " and a rustc warning would be triggered, see #15301"] fn fn_is_externally_exported (cx : & LateContext < '_ > , def_id : DefId) -> bool { let attrs = cx . tcx . codegen_fn_attrs (def_id) ; attrs . contains_extern_indicator () }
};
}
