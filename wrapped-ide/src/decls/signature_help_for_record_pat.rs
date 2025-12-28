macro_rules! deps {
    () => {
        SignatureHelp!();
    };
}

macro_rules! signature_help_for_record_pat {
    () => {
        deps!();
        fn signature_help_for_record_pat (sema : & Semantics < '_ , RootDatabase > , record : ast :: RecordPat , token : SyntaxToken , edition : Edition , display_target : DisplayTarget ,) -> Option < SignatureHelp > { signature_help_for_record_ (sema , record . record_pat_field_list () ? . syntax () . children_with_tokens () , & record . path () ? , record . record_pat_field_list () ? . fields () . filter_map (| field | sema . resolve_record_pat_field (& field)) , token , edition , display_target ,) }
    };
}

signature_help_for_record_pat!();