macro_rules! deps {
    () => {
        SignatureHelp!();
    };
}

macro_rules! signature_help_for_tuple_pat_ish {
    () => {
        deps!();
        fn signature_help_for_tuple_pat_ish < 'db > (db : & 'db RootDatabase , mut res : SignatureHelp , pat : & SyntaxNode , token : SyntaxToken , mut field_pats : AstChildren < ast :: Pat > , fields : impl ExactSizeIterator < Item = hir :: Type < 'db > > , display_target : DisplayTarget ,) -> SignatureHelp { let rest_pat = field_pats . find (| it | matches ! (it , ast :: Pat :: RestPat (_))) ; let is_left_of_rest_pat = rest_pat . is_none_or (| it | token . text_range () . start () < it . syntax () . text_range () . end ()) ; let commas = pat . children_with_tokens () . filter_map (NodeOrToken :: into_token) . filter (| t | t . kind () == T ! [,]) ; res . active_parameter = { Some (if is_left_of_rest_pat { commas . take_while (| t | t . text_range () . start () <= token . text_range () . start ()) . count () } else { let n_commas = commas . collect :: < Vec < _ > > () . into_iter () . rev () . take_while (| t | t . text_range () . start () > token . text_range () . start ()) . count () ; fields . len () . saturating_sub (1) . saturating_sub (n_commas) }) } ; let mut buf = String :: new () ; for ty in fields { format_to ! (buf , "{}" , ty . display_truncated (db , Some (20) , display_target)) ; res . push_call_param (& buf) ; buf . clear () ; } res . signature . push (')') ; res }
    };
}

signature_help_for_tuple_pat_ish!()