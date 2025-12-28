macro_rules! SupertraitItemShadowee {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum SupertraitItemShadowee { # [note (hir_typeck_supertrait_item_shadowee)] Labeled { # [primary_span] span : Span , supertrait : Symbol , } , # [note (hir_typeck_supertrait_item_multiple_shadowee)] Several { # [primary_span] spans : MultiSpan , traits : DiagSymbolList , } , }
    };
}

SupertraitItemShadowee!()