macro_rules! deps {
    () => {
        ProcMacro!();
        ServerError!();
        ProcMacroKind!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl ProcMacro { # [doc = " Returns the name of the procedural macro."] pub fn name (& self) -> & str { & self . name } # [doc = " Returns the type of procedural macro."] pub fn kind (& self) -> ProcMacroKind { self . kind } fn needs_fixup_change (& self) -> bool { let version = self . process . version () ; (version :: RUST_ANALYZER_SPAN_SUPPORT .. version :: HASHED_AST_ID) . contains (& version) } # [doc = " On some server versions, the fixup ast id is different than ours. So change it to match."] fn change_fixup_to_match_old_server (& self , tt : & mut tt :: TopSubtree < Span >) { const OLD_FIXUP_AST_ID : ErasedFileAstId = ErasedFileAstId :: from_raw (! 0 - 1) ; let change_ast_id = | ast_id : & mut ErasedFileAstId | { if * ast_id == FIXUP_ERASED_FILE_AST_ID_MARKER { * ast_id = OLD_FIXUP_AST_ID ; } else if * ast_id == OLD_FIXUP_AST_ID { * ast_id = FIXUP_ERASED_FILE_AST_ID_MARKER ; } } ; for tt in & mut tt . 0 { match tt { tt :: TokenTree :: Leaf (tt :: Leaf :: Ident (tt :: Ident { span , .. })) | tt :: TokenTree :: Leaf (tt :: Leaf :: Literal (tt :: Literal { span , .. })) | tt :: TokenTree :: Leaf (tt :: Leaf :: Punct (tt :: Punct { span , .. })) => { change_ast_id (& mut span . anchor . ast_id) ; } tt :: TokenTree :: Subtree (subtree) => { change_ast_id (& mut subtree . delimiter . open . anchor . ast_id) ; change_ast_id (& mut subtree . delimiter . close . anchor . ast_id) ; } } } } # [doc = " Expands the procedural macro by sending an expansion request to the server."] # [doc = " This includes span information and environmental context."] pub fn expand (& self , subtree : tt :: SubtreeView < '_ , Span > , attr : Option < tt :: SubtreeView < '_ , Span > > , env : Vec < (String , String) > , def_site : Span , call_site : Span , mixed_site : Span , current_dir : String ,) -> Result < Result < tt :: TopSubtree < Span > , String > , ServerError > { let (mut subtree , mut attr) = (subtree , attr) ; let (mut subtree_changed , mut attr_changed) ; if self . needs_fixup_change () { subtree_changed = tt :: TopSubtree :: from_subtree (subtree) ; self . change_fixup_to_match_old_server (& mut subtree_changed) ; subtree = subtree_changed . view () ; if let Some (attr) = & mut attr { attr_changed = tt :: TopSubtree :: from_subtree (* attr) ; self . change_fixup_to_match_old_server (& mut attr_changed) ; * attr = attr_changed . view () ; } } legacy_protocol :: expand (self , subtree , attr , env , def_site , call_site , mixed_site , current_dir ,) } }
    };
}

impl_13!()