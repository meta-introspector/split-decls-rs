// Generated macro for is_in_cfg_test (function)
macro_rules! Depcrateis_in_cfg_test {
() => {
// Module: crate
// Provides: {"is_in_cfg_test"}
// Dependencies: {}
# [doc = " Checks if any parent node of `HirId` has `#[cfg(test)]` attribute applied"] pub fn is_in_cfg_test (tcx : TyCtxt < '_ > , id : HirId) -> bool { tcx . hir_parent_id_iter (id) . any (| parent_id | is_cfg_test (tcx , parent_id)) }
};
}
