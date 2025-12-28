macro_rules! MultipleSupertraitUpcastable {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_multiple_supertrait_upcastable)] pub (crate) struct MultipleSupertraitUpcastable { pub ident : Ident , }
    };
}

MultipleSupertraitUpcastable!()