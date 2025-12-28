macro_rules! UnusedAllocationMutDiag {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_unused_allocation_mut)] pub (crate) struct UnusedAllocationMutDiag ;
    };
}

UnusedAllocationMutDiag!()