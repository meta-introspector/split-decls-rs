// Generated macro for AwaitOnlyInAsyncFnAndBlocks (struct)
macro_rules! Depcrate_errorsAwaitOnlyInAsyncFnAndBlocks {
() => {
// Module: crate::errors
// Provides: {"AwaitOnlyInAsyncFnAndBlocks"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (ast_lowering_await_only_in_async_fn_and_blocks , code = E0728)] pub (crate) struct AwaitOnlyInAsyncFnAndBlocks { # [primary_span] # [label] pub await_kw_span : Span , # [label (ast_lowering_this_not_async)] pub item_span : Option < Span > , }
};
}
