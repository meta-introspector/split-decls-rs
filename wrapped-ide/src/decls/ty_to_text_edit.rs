macro_rules! deps {
    () => {
        LazyProperty!();
        InlayHintsConfig!();
    };
}

macro_rules! ty_to_text_edit {
    () => {
        deps!();
        fn ty_to_text_edit (sema : & Semantics < '_ , RootDatabase > , config : & InlayHintsConfig < '_ > , node_for_hint : & SyntaxNode , ty : & hir :: Type < '_ > , offset_to_insert_ty : TextSize , additional_edits : & dyn Fn (& mut TextEditBuilder) , prefix : impl Into < String > ,) -> Option < LazyProperty < TextEdit > > { let rendered = sema . scope (node_for_hint) . and_then (| scope | ty . display_source_code (scope . db , scope . module () . into () , false) . ok ()) ? ; Some (config . lazy_text_edit (| | { let mut builder = TextEdit :: builder () ; builder . insert (offset_to_insert_ty , prefix . into ()) ; builder . insert (offset_to_insert_ty , rendered) ; additional_edits (& mut builder) ; builder . finish () })) }
    };
}

ty_to_text_edit!()