macro_rules! deps {
    () => {
        ExpansionSpanMap!();
        ExpandDatabase!();
    };
}

macro_rules! prettify_macro_expansion {
    () => {
        deps!();
        # [doc = " Inserts whitespace and replaces `$crate` in macro expansions."] # [expect (deprecated)] pub fn prettify_macro_expansion (db : & dyn ExpandDatabase , syn : SyntaxNode , span_map : & ExpansionSpanMap , target_crate_id : Crate ,) -> SyntaxNode { let span_offset = syn . text_range () . start () ; let target_crate = target_crate_id . data (db) ; let mut syntax_ctx_id_to_dollar_crate_replacement = FxHashMap :: default () ; syntax_bridge :: prettify_macro_expansion :: prettify_macro_expansion (syn , & mut | dollar_crate | { let ctx = span_map . span_at (dollar_crate . text_range () . start () + span_offset) . ctx ; let replacement = syntax_ctx_id_to_dollar_crate_replacement . entry (ctx) . or_insert_with (| | { let macro_call_id = ctx . outer_expn (db) . expect ("`$crate` cannot come from `SyntaxContextId::ROOT`") ; let macro_call = db . lookup_intern_macro_call (macro_call_id . into ()) ; let macro_def_crate = macro_call . def . krate ; if target_crate_id == macro_def_crate { make :: tokens :: crate_kw () } else if let Some (dep) = target_crate . dependencies . iter () . find (| dep | dep . crate_id == macro_def_crate) { make :: tokens :: ident (dep . name . as_str ()) } else if let Some (crate_name) = & macro_def_crate . extra_data (db) . display_name { make :: tokens :: ident (crate_name . crate_name () . as_str ()) } else { dollar_crate . clone () } }) ; if replacement . text () == "$crate" { return None ; } let parent = replacement . parent () . unwrap () . clone_subtree () . clone_for_update () ; parent . children_with_tokens () . filter_map (NodeOrToken :: into_token) . find (| it | it . kind () == replacement . kind ()) } , | _ | () ,) }
    };
}

prettify_macro_expansion!()