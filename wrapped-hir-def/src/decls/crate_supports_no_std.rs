macro_rules! deps {
    () => {
        DefDatabase!();
    };
}

macro_rules! crate_supports_no_std {
    () => {
        deps!();
        fn crate_supports_no_std (db : & dyn DefDatabase , crate_id : Crate) -> bool { let file = crate_id . data (db) . root_file_id (db) ; let item_tree = db . file_item_tree (file . into ()) ; let attrs = item_tree . top_level_raw_attrs () ; for attr in & * * attrs { match attr . path () . as_ident () { Some (ident) if * ident == sym :: no_std => return true , Some (ident) if * ident == sym :: cfg_attr => { } _ => continue , } let tt = match attr . token_tree_value () { Some (tt) => tt . token_trees () , None => continue , } ; let segments = tt . split (| tt | matches ! (tt , tt :: TtElement :: Leaf (tt :: Leaf :: Punct (p)) if p . char == ',')) ; for output in segments . skip (1) { match output . flat_tokens () { [tt :: TokenTree :: Leaf (tt :: Leaf :: Ident (ident))] if ident . sym == sym :: no_std => { return true ; } _ => { } } } } false }
    };
}

crate_supports_no_std!()