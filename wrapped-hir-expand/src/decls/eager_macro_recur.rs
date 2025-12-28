macro_rules! deps {
    () => {
        ExpansionSpanMap!();
        ProcMacro!();
        MacroDefKind!();
        InFile!();
        ExpandResult!();
        ExpandError!();
        EagerCallBackFn!();
        ExpandDatabase!();
        ModPath!();
        MacroDefId!();
    };
}

macro_rules! eager_macro_recur {
    () => {
        deps!();
        fn eager_macro_recur (db : & dyn ExpandDatabase , span_map : & ExpansionSpanMap , expanded_map : & mut ExpansionSpanMap , mut offset : TextSize , curr : InFile < SyntaxNode > , krate : Crate , call_site : SyntaxContext , macro_resolver : & dyn Fn (& ModPath) -> Option < MacroDefId > , eager_callback : EagerCallBackFn < '_ > ,) -> ExpandResult < Option < (SyntaxNode , TextSize) > > { let original = curr . value . clone_for_update () ; let mut replacements = Vec :: new () ; let mut error = None ; let mut children = original . preorder_with_tokens () ; while let Some (child) = children . next () { let call = match child { WalkEvent :: Enter (SyntaxElement :: Node (child)) => match ast :: MacroCall :: cast (child) { Some (it) => { children . skip_subtree () ; it } _ => continue , } , WalkEvent :: Enter (_) => continue , WalkEvent :: Leave (child) => { if let SyntaxElement :: Token (t) = child { let start = t . text_range () . start () ; offset += t . text_range () . len () ; expanded_map . push (offset , span_map . span_at (start)) ; } continue ; } } ; let def = match call . path () . and_then (| path | { ModPath :: from_src (db , path , & mut | range | span_map . span_at (range . start ()) . ctx) }) { Some (path) => match macro_resolver (& path) { Some (def) => def , None => { let edition = krate . data (db) . edition ; error = Some (ExpandError :: other (span_map . span_at (call . syntax () . text_range () . start ()) , format ! ("unresolved macro {}" , path . display (db , edition)) ,)) ; offset += call . syntax () . text_range () . len () ; continue ; } } , None => { error = Some (ExpandError :: other (span_map . span_at (call . syntax () . text_range () . start ()) , "malformed macro invocation" ,)) ; offset += call . syntax () . text_range () . len () ; continue ; } } ; let ast_id = db . ast_id_map (curr . file_id) . ast_id (& call) ; let ExpandResult { value , err } = match def . kind { MacroDefKind :: BuiltInEager (..) => { let ExpandResult { value , err } = expand_eager_macro_input (db , krate , & call , curr . with_value (ast_id) , def , call_site , macro_resolver , eager_callback ,) ; match value { Some (call_id) => { eager_callback (curr . with_value (ast_id) . map (| ast_id | (AstPtr :: new (& call) , ast_id)) , call_id ,) ; let ExpandResult { value : (parse , map) , err : err2 } = db . parse_macro_expansion (call_id) ; map . iter () . for_each (| (o , span) | expanded_map . push (o + offset , span)) ; let syntax_node = parse . syntax_node () ; ExpandResult { value : Some ((syntax_node . clone_for_update () , offset + syntax_node . text_range () . len () ,)) , err : err . or (err2) , } } None => ExpandResult { value : None , err } , } } MacroDefKind :: Declarative (_) | MacroDefKind :: BuiltIn (..) | MacroDefKind :: BuiltInAttr (..) | MacroDefKind :: BuiltInDerive (..) | MacroDefKind :: ProcMacro (..) => { let ExpandResult { value : (parse , tm) , err } = lazy_expand (db , & def , & call , curr . with_value (ast_id) , krate , call_site , eager_callback ,) ; let ExpandResult { value , err : error } = eager_macro_recur (db , & tm , expanded_map , offset , parse . as_ref () . map (| it | it . syntax_node ()) , krate , call_site , macro_resolver , eager_callback ,) ; let err = err . or (error) ; ExpandResult { value , err } } } ; if err . is_some () { error = err ; } if call . syntax () == & original { return ExpandResult { value , err : error } ; } match value { Some ((insert , new_offset)) => { replacements . push ((call , insert)) ; offset = new_offset ; } None => offset += call . syntax () . text_range () . len () , } } replacements . into_iter () . rev () . for_each (| (old , new) | ted :: replace (old . syntax () , new)) ; ExpandResult { value : Some ((original , offset)) , err : error } }
    };
}

eager_macro_recur!();