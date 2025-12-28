macro_rules! deps {
    () => {
        NonBindingLetSub!();
    };
}

macro_rules! NonBindingLet {
    () => {
        deps!();
        # [derive (LintDiagnostic)] pub (crate) enum NonBindingLet { # [diag (lint_non_binding_let_on_sync_lock)] SyncLock { # [label] pat : Span , # [subdiagnostic] sub : NonBindingLetSub , } , # [diag (lint_non_binding_let_on_drop_type)] DropType { # [subdiagnostic] sub : NonBindingLetSub , } , }
    };
}

NonBindingLet!();