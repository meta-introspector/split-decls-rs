macro_rules! AwaitOnlyInAsyncFnAndBlocks {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_await_only_in_async_fn_and_blocks , code = E0728)] pub (crate) struct AwaitOnlyInAsyncFnAndBlocks { # [primary_span] # [label] pub await_kw_span : Span , # [label (ast_lowering_this_not_async)] pub item_span : Option < Span > , }
    };
}

AwaitOnlyInAsyncFnAndBlocks!()