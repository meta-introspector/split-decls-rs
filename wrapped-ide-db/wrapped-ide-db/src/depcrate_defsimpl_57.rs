// Generated macro for impl_57 (impl)
macro_rules! Depcrate_defsimpl_57 {
() => {
// Module: crate::defs
// Provides: {"impl_57"}
// Dependencies: {}
impl OperatorClass { pub fn classify_range_pat (sema : & Semantics < '_ , RootDatabase > , range_pat : & ast :: RangePat ,) -> Option < OperatorClass > { sema . resolve_range_pat (range_pat) . map (OperatorClass :: Range) } pub fn classify_range_expr (sema : & Semantics < '_ , RootDatabase > , range_expr : & ast :: RangeExpr ,) -> Option < OperatorClass > { sema . resolve_range_expr (range_expr) . map (OperatorClass :: Range) } pub fn classify_await (sema : & Semantics < '_ , RootDatabase > , await_expr : & ast :: AwaitExpr ,) -> Option < OperatorClass > { sema . resolve_await_to_poll (await_expr) . map (OperatorClass :: Await) } pub fn classify_prefix (sema : & Semantics < '_ , RootDatabase > , prefix_expr : & ast :: PrefixExpr ,) -> Option < OperatorClass > { sema . resolve_prefix_expr (prefix_expr) . map (OperatorClass :: Prefix) } pub fn classify_try (sema : & Semantics < '_ , RootDatabase > , try_expr : & ast :: TryExpr ,) -> Option < OperatorClass > { sema . resolve_try_expr (try_expr) . map (OperatorClass :: Try) } pub fn classify_index (sema : & Semantics < '_ , RootDatabase > , index_expr : & ast :: IndexExpr ,) -> Option < OperatorClass > { sema . resolve_index_expr (index_expr) . map (OperatorClass :: Index) } pub fn classify_bin (sema : & Semantics < '_ , RootDatabase > , bin_expr : & ast :: BinExpr ,) -> Option < OperatorClass > { sema . resolve_bin_expr (bin_expr) . map (OperatorClass :: Bin) } }
};
}
