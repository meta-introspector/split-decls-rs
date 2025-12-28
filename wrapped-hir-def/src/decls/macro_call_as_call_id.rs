macro_rules! deps {
    () => {
        MacroCall!();
        UnresolvedMacro!();
    };
}

macro_rules! macro_call_as_call_id {
    () => {
        deps!();
        pub fn macro_call_as_call_id (db : & dyn ExpandDatabase , ast_id : AstId < ast :: MacroCall > , path : & ModPath , call_site : SyntaxContext , expand_to : ExpandTo , krate : Crate , resolver : impl Fn (& ModPath) -> Option < MacroDefId > + Copy , eager_callback : & mut dyn FnMut (InFile < (syntax :: AstPtr < ast :: MacroCall > , span :: FileAstId < ast :: MacroCall >) > , MacroCallId ,) ,) -> Result < ExpandResult < Option < MacroCallId > > , UnresolvedMacro > { let def = resolver (path) . ok_or_else (| | UnresolvedMacro { path : path . clone () }) ? ; let res = match def . kind { MacroDefKind :: BuiltInEager (..) => expand_eager_macro_input (db , krate , & ast_id . to_node (db) , ast_id , def , call_site , & | path | resolver (path) . filter (MacroDefId :: is_fn_like) , eager_callback ,) , _ if def . is_fn_like () => ExpandResult { value : Some (def . make_call (db , krate , MacroCallKind :: FnLike { ast_id , expand_to , eager : None } , call_site ,)) , err : None , } , _ => return Err (UnresolvedMacro { path : path . clone () }) , } ; Ok (res) }
    };
}

macro_call_as_call_id!();