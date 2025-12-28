macro_rules! UnusedAllocationDiag {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_unused_allocation)] pub (crate) struct UnusedAllocationDiag ;
    };
}

UnusedAllocationDiag!()