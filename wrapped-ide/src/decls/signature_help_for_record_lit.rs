macro_rules! deps {
    () => {
        SignatureHelp!();
    };
}

macro_rules! signature_help_for_record_lit {
    () => {
        deps!();
        fn signature_help_for_record_lit (sema : & Semantics < '_ , RootDatabase > , record : ast :: RecordExpr , token : SyntaxToken , edition : Edition , display_target : DisplayTarget ,) -> Option < SignatureHelp > { signature_help_for_record_ (sema , record . record_expr_field_list () ? . syntax () . children_with_tokens () , & record . path () ? , record . record_expr_field_list () ? . fields () . filter_map (| field | sema . resolve_record_field (& field)) . map (| (field , _ , ty) | (field , ty)) , token , edition , display_target ,) }
    };
}

signature_help_for_record_lit!()