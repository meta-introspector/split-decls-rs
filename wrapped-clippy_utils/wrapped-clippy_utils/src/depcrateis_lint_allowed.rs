// Generated macro for is_lint_allowed (function)
macro_rules! Depcrateis_lint_allowed {
() => {
// Module: crate
// Provides: {"is_lint_allowed"}
// Dependencies: {}
# [doc = " Returns `true` if the lint is allowed in the current context. This is useful for"] # [doc = " skipping long running code when it's unnecessary"] # [doc = ""] # [doc = " This function should check the lint level for the same node, that the lint will"] # [doc = " be emitted at. If the information is buffered to be emitted at a later point, please"] # [doc = " make sure to use `span_lint_hir` functions to emit the lint. This ensures that"] # [doc = " expectations at the checked nodes will be fulfilled."] pub fn is_lint_allowed (cx : & LateContext < '_ > , lint : & 'static Lint , id : HirId) -> bool { cx . tcx . lint_level_at_node (lint , id) . level == Level :: Allow }
};
}
