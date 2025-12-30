// Generated macro for is_cfg_test (function)
macro_rules! Depcrateis_cfg_test {
() => {
// Module: crate
// Provides: {"is_cfg_test"}
// Dependencies: {}
# [doc = " Checks if `id` has a `#[cfg(test)]` attribute applied"] # [doc = ""] # [doc = " This only checks directly applied attributes, to see if a node is inside a `#[cfg(test)]` parent"] # [doc = " use [`is_in_cfg_test`]"] pub fn is_cfg_test (tcx : TyCtxt < '_ > , id : HirId) -> bool { tcx . hir_attrs (id) . iter () . any (| attr | { if attr . has_name (sym :: cfg_trace) && let Some (items) = attr . meta_item_list () && let [item] = & * items && item . has_name (sym :: test) { true } else { false } }) }
};
}
