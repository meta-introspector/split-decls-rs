macro_rules! deps {
    () => {
        ReplaceWithName!();
    };
}

macro_rules! SelfCtorFromOuterItemLint {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (hir_typeck_self_ctor_from_outer_item)] pub (crate) struct SelfCtorFromOuterItemLint { # [label] pub impl_span : Span , # [subdiagnostic] pub sugg : Option < ReplaceWithName > , }
    };
}

SelfCtorFromOuterItemLint!()