macro_rules! deps {
    () => {
        Literal!();
        CfgAtom!();
        CfgExpr!();
    };
}

macro_rules! next_cfg_expr {
    () => {
        deps!();
        # [cfg (feature = "tt")] fn next_cfg_expr < S : Copy > (it : & mut tt :: iter :: TtIter < '_ , S >) -> Option < CfgExpr > { use intern :: sym ; use tt :: iter :: TtElement ; let name = match it . next () { None => return None , Some (TtElement :: Leaf (tt :: Leaf :: Ident (ident))) => ident . sym . clone () , Some (_) => return Some (CfgExpr :: Invalid) , } ; let ret = match it . peek () { Some (TtElement :: Leaf (tt :: Leaf :: Punct (punct))) if punct . char == '=' && (punct . spacing == tt :: Spacing :: Alone || it . remaining () . flat_tokens () . get (1) . is_none_or (| peek2 | { ! matches ! (peek2 , tt :: TokenTree :: Leaf (tt :: Leaf :: Punct (_))) })) => { match it . remaining () . flat_tokens () . get (1) { Some (tt :: TokenTree :: Leaf (tt :: Leaf :: Literal (literal))) => { it . next () ; it . next () ; CfgAtom :: KeyValue { key : name , value : literal . symbol . clone () } . into () } _ => return Some (CfgExpr :: Invalid) , } } Some (TtElement :: Subtree (_ , mut sub_it)) => { it . next () ; let mut subs = std :: iter :: from_fn (| | next_cfg_expr (& mut sub_it)) ; match name { s if s == sym :: all => CfgExpr :: All (subs . collect ()) , s if s == sym :: any => CfgExpr :: Any (subs . collect ()) , s if s == sym :: not => { CfgExpr :: Not (Box :: new (subs . next () . unwrap_or (CfgExpr :: Invalid))) } _ => CfgExpr :: Invalid , } } _ => CfgAtom :: Flag (name) . into () , } ; if let Some (TtElement :: Leaf (tt :: Leaf :: Punct (punct))) = it . peek () && punct . char == ',' { it . next () ; } Some (ret) }
    };
}

next_cfg_expr!();