// Generated macro for is_in_test (function)
macro_rules! Depcrateis_in_test {
() => {
// Module: crate
// Provides: {"is_in_test"}
// Dependencies: {}
# [doc = " Checks if the node is in a `#[test]` function or has any parent node marked `#[cfg(test)]`"] pub fn is_in_test (tcx : TyCtxt < '_ > , hir_id : HirId) -> bool { is_in_test_function (tcx , hir_id) || is_in_cfg_test (tcx , hir_id) }
};
}
