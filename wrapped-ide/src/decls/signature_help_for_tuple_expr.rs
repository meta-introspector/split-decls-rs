macro_rules! deps {
    () => {
        SignatureHelp!();
    };
}

macro_rules! signature_help_for_tuple_expr {
    () => {
        deps!();
        fn signature_help_for_tuple_expr (sema : & Semantics < '_ , RootDatabase > , expr : ast :: TupleExpr , token : SyntaxToken , display_target : DisplayTarget ,) -> Option < SignatureHelp > { let active_parameter = Some (expr . syntax () . children_with_tokens () . filter_map (NodeOrToken :: into_token) . filter (| t | t . kind () == T ! [,]) . take_while (| t | t . text_range () . start () <= token . text_range () . start ()) . count () ,) ; let db = sema . db ; let mut res = SignatureHelp { doc : None , signature : String :: from ('(') , parameters : vec ! [] , active_parameter , } ; let expr = sema . type_of_expr (& expr . into ()) ? ; let fields = expr . original . tuple_fields (db) ; let mut buf = String :: new () ; for ty in fields { format_to ! (buf , "{}" , ty . display_truncated (db , Some (20) , display_target)) ; res . push_call_param (& buf) ; buf . clear () ; } res . signature . push (')') ; Some (res) }
    };
}

signature_help_for_tuple_expr!();