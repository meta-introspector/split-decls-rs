macro_rules! deps {
    () => {
        DotAccess!();
        CompletionItem!();
        RenderContext!();
        CompletionRelevance!();
        DotAccessKind!();
    };
}

macro_rules! render_field {
    () => {
        deps!();
        pub (crate) fn render_field (ctx : RenderContext < '_ > , dot_access : & DotAccess < '_ > , receiver : Option < SmolStr > , field : hir :: Field , ty : & hir :: Type < '_ > ,) -> CompletionItem { let db = ctx . db () ; let is_deprecated = ctx . is_deprecated (field) ; let name = field . name (db) ; let (name , escaped_name) = (name . as_str () . to_smolstr () , name . display_no_db (ctx . completion . edition) . to_smolstr ()) ; let mut item = CompletionItem :: new (SymbolKind :: Field , ctx . source_range () , field_with_receiver (receiver . as_deref () , & name) , ctx . completion . edition ,) ; item . set_relevance (CompletionRelevance { type_match : compute_type_match (ctx . completion , ty) , exact_name_match : compute_exact_name_match (ctx . completion , & name) , is_skipping_completion : receiver . is_some () , .. CompletionRelevance :: default () }) ; item . detail (ty . display (db , ctx . completion . display_target) . to_string ()) . set_documentation (field . docs (db)) . set_deprecated (is_deprecated) . lookup_by (name) ; let is_field_access = matches ! (dot_access . kind , DotAccessKind :: Field { .. }) ; if ! is_field_access || ty . is_fn () || ty . is_closure () { let mut builder = TextEdit :: builder () ; builder . replace (ctx . source_range () , field_with_receiver (receiver . as_deref () , & escaped_name) . into () ,) ; let expected_fn_type = ctx . completion . expected_type . as_ref () . is_some_and (| ty | ty . is_fn () || ty . is_closure ()) ; if ! expected_fn_type && let Some (receiver) = & dot_access . receiver && let Some (receiver) = ctx . completion . sema . original_ast_node (receiver . clone ()) { builder . insert (receiver . syntax () . text_range () . start () , "(" . to_owned ()) ; builder . insert (ctx . source_range () . end () , ")" . to_owned ()) ; let is_parens_needed = ! matches ! (dot_access . kind , DotAccessKind :: Method) ; if is_parens_needed { builder . insert (ctx . source_range () . end () , "()" . to_owned ()) ; } } item . text_edit (builder . finish ()) ; } else { item . insert_text (field_with_receiver (receiver . as_deref () , & escaped_name)) ; } if let Some (receiver) = & dot_access . receiver && let Some (original) = ctx . completion . sema . original_ast_node (receiver . clone ()) && let Some (ref_mode) = compute_ref_match (ctx . completion , ty) { item . ref_match (ref_mode , original . syntax () . text_range () . start ()) ; } item . doc_aliases (ctx . doc_aliases) ; item . build (db) }
    };
}

render_field!()