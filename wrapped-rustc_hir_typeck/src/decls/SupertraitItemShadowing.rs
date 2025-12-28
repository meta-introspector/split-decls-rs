macro_rules! deps {
    () => {
        SupertraitItemShadower!();
        SupertraitItemShadowee!();
    };
}

macro_rules! SupertraitItemShadowing {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (hir_typeck_supertrait_item_shadowing)] pub (crate) struct SupertraitItemShadowing { pub item : Symbol , pub subtrait : Symbol , # [subdiagnostic] pub shadower : SupertraitItemShadower , # [subdiagnostic] pub shadowee : SupertraitItemShadowee , }
    };
}

SupertraitItemShadowing!()