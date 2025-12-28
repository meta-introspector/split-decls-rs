macro_rules! deps {
    () => {
        DocAtom!();
        Literal!();
        DocExpr!();
    };
}

macro_rules! next_doc_expr {
    () => {
        deps!();
        fn next_doc_expr < S : Copy > (mut it : TtIter < '_ , S >) -> Option < DocExpr > { let name = match it . next () { None => return None , Some (TtElement :: Leaf (tt :: Leaf :: Ident (ident))) => ident . sym . clone () , Some (_) => return Some (DocExpr :: Invalid) , } ; let ret = match it . peek () { Some (TtElement :: Leaf (tt :: Leaf :: Punct (punct))) if punct . char == '=' => { it . next () ; match it . next () { Some (TtElement :: Leaf (tt :: Leaf :: Literal (tt :: Literal { symbol : text , kind : tt :: LitKind :: Str , .. }))) => DocAtom :: KeyValue { key : name , value : text . clone () } . into () , _ => return Some (DocExpr :: Invalid) , } } Some (TtElement :: Subtree (_ , subtree_iter)) => { it . next () ; let subs = parse_comma_sep (subtree_iter) ; match & name { s if * s == sym :: alias => DocExpr :: Alias (subs) , _ => DocExpr :: Invalid , } } _ => DocAtom :: Flag (name) . into () , } ; Some (ret) }
    };
}

next_doc_expr!()