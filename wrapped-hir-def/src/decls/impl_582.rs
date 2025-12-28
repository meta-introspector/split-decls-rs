macro_rules! deps {
    () => {
        IdentityWhenValidProcMacroExpander!();
    };
}

macro_rules! impl_582 {
    () => {
        deps!();
        impl ProcMacroExpander for IdentityWhenValidProcMacroExpander { fn expand (& self , subtree : & TopSubtree , _ : Option < & TopSubtree > , _ : & base_db :: Env , _ : Span , _ : Span , _ : Span , _ : String ,) -> Result < TopSubtree , ProcMacroExpansionError > { let (parse , _) = syntax_bridge :: token_tree_to_syntax_node (subtree , syntax_bridge :: TopEntryPoint :: MacroItems , & mut | _ | span :: Edition :: CURRENT , span :: Edition :: CURRENT ,) ; if parse . errors () . is_empty () { Ok (subtree . clone ()) } else { panic ! ("got invalid macro input: {:?}" , parse . errors ()) ; } } fn eq_dyn (& self , other : & dyn ProcMacroExpander) -> bool { other . type_id () == TypeId :: of :: < Self > () } }
    };
}

impl_582!();