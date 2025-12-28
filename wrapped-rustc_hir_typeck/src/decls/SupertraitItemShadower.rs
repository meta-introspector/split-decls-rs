macro_rules! SupertraitItemShadower {
    () => {
        # [derive (Subdiagnostic)] # [note (hir_typeck_supertrait_item_shadower)] pub (crate) struct SupertraitItemShadower { pub subtrait : Symbol , # [primary_span] pub span : Span , }
    };
}

SupertraitItemShadower!()