macro_rules! deps {
    () => {
        SupertraitItemShadowee!();
    };
}

macro_rules! SupertraitItemShadowing {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (hir_analysis_supertrait_item_shadowing)] pub (crate) struct SupertraitItemShadowing { pub item : Symbol , pub subtrait : Symbol , # [subdiagnostic] pub shadowee : SupertraitItemShadowee , }
    };
}

SupertraitItemShadowing!()