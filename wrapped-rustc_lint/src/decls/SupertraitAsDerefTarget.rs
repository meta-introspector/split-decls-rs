macro_rules! deps {
    () => {
        SupertraitAsDerefTargetLabel!();
    };
}

macro_rules! SupertraitAsDerefTarget {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_supertrait_as_deref_target)] pub (crate) struct SupertraitAsDerefTarget < 'a > { pub self_ty : Ty < 'a > , pub supertrait_principal : PolyExistentialTraitRef < 'a > , pub target_principal : PolyExistentialTraitRef < 'a > , # [label] pub label : Span , # [subdiagnostic] pub label2 : Option < SupertraitAsDerefTargetLabel > , }
    };
}

SupertraitAsDerefTarget!()