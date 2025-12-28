macro_rules! deps {
    () => {
        HasAttrs!();
        DocLinkDef!();
    };
}

macro_rules! resolve_doc_path_on {
    () => {
        deps!();
        # [doc = " Resolves the item `link` points to in the scope of `def`."] pub fn resolve_doc_path_on (db : & dyn HirDatabase , def : impl HasAttrs + Copy , link : & str , ns : Option < Namespace > , is_inner_doc : bool ,) -> Option < DocLinkDef > { resolve_doc_path_on_ (db , link , def . attr_id () , ns , is_inner_doc) }
    };
}

resolve_doc_path_on!()