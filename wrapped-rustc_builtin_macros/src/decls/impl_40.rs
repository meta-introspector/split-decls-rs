macro_rules! deps {
    () => {
        Expander!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl MultiItemModifier for Expander { fn expand (& self , ecx : & mut ExtCtxt < '_ > , span : Span , meta_item : & ast :: MetaItem , item : Annotatable , _ : bool ,) -> ExpandResult < Vec < Annotatable > , Annotatable > { let sess = ecx . sess ; if report_bad_target (sess , & item , span) . is_err () { return ExpandResult :: Ready (vec ! [item]) ; } let (sess , features) = (ecx . sess , ecx . ecfg . features) ; let result = ecx . resolver . resolve_derives (ecx . current_expansion . id , ecx . force_mode , & | | { let template = AttributeTemplate { list : Some (& ["Trait1, Trait2, ..."]) , .. Default :: default () } ; validate_attr :: check_builtin_meta_item (& sess . psess , meta_item , ast :: AttrStyle :: Outer , sym :: derive , template , true ,) ; let mut resolutions = match & meta_item . kind { MetaItemKind :: List (list) => { list . iter () . filter_map (| meta_item_inner | match meta_item_inner { MetaItemInner :: MetaItem (meta) => Some (meta) , MetaItemInner :: Lit (lit) => { report_unexpected_meta_item_lit (sess , lit) ; None } }) . map (| meta | { report_path_args (sess , meta) ; meta . path . clone () }) . map (| path | DeriveResolution { path , item : dummy_annotatable () , exts : None , is_const : self . is_const , }) . collect () } _ => vec ! [] , } ; match & mut resolutions [..] { [] => { } [first , others @ ..] => { first . item = cfg_eval (sess , features , item . clone () , ecx . current_expansion . lint_node_id ,) ; for other in others { other . item = first . item . clone () ; } } } resolutions }) ; match result { Ok (()) => ExpandResult :: Ready (vec ! [item]) , Err (Indeterminate) => ExpandResult :: Retry (item) , } } }
    };
}

impl_40!();