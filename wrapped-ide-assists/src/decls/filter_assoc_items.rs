macro_rules! deps {
    () => {
        DefaultMethods!();
        IgnoreAssocItems!();
    };
}

macro_rules! filter_assoc_items {
    () => {
        deps!();
        pub fn filter_assoc_items (sema : & Semantics < '_ , RootDatabase > , items : & [hir :: AssocItem] , default_methods : DefaultMethods , ignore_items : IgnoreAssocItems ,) -> Vec < InFile < ast :: AssocItem > > { return items . iter () . copied () . filter (| assoc_item | { if ignore_items == IgnoreAssocItems :: DocHiddenAttrPresent && assoc_item . attrs (sema . db) . has_doc_hidden () { if let hir :: AssocItem :: Function (f) = assoc_item && ! f . has_body (sema . db) { return true ; } return false ; } true }) . filter_map (| assoc_item | { let item = match assoc_item { hir :: AssocItem :: Function (it) => sema . source (it) ? . map (ast :: AssocItem :: Fn) , hir :: AssocItem :: TypeAlias (it) => sema . source (it) ? . map (ast :: AssocItem :: TypeAlias) , hir :: AssocItem :: Const (it) => sema . source (it) ? . map (ast :: AssocItem :: Const) , } ; Some (item) }) . filter (has_def_name) . filter (| it | match & it . value { ast :: AssocItem :: Fn (def) => matches ! ((default_methods , def . body ()) , (DefaultMethods :: Only , Some (_)) | (DefaultMethods :: No , None)) , ast :: AssocItem :: Const (def) => matches ! ((default_methods , def . body ()) , (DefaultMethods :: Only , Some (_)) | (DefaultMethods :: No , None)) , _ => default_methods == DefaultMethods :: No , }) . collect () ; fn has_def_name (item : & InFile < ast :: AssocItem >) -> bool { match & item . value { ast :: AssocItem :: Fn (def) => def . name () , ast :: AssocItem :: TypeAlias (def) => def . name () , ast :: AssocItem :: Const (def) => def . name () , ast :: AssocItem :: MacroCall (_) => None , } . is_some () } }
    };
}

filter_assoc_items!();