macro_rules! deps {
    () => {
        AsyncClosureSugg!();
    };
}

macro_rules! ClosureReturningAsyncBlock {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_closure_returning_async_block)] struct ClosureReturningAsyncBlock { # [label] async_decl_span : Span , # [subdiagnostic] sugg : AsyncClosureSugg , }
    };
}

ClosureReturningAsyncBlock!()