mkuse!{use rustc_ast as ast ;}
mkuse!{use rustc_ast :: { GenericParamKind , ItemKind , MetaItemInner , MetaItemKind , StmtKind } ;}
mkuse!{use rustc_attr_parsing :: validate_attr ;}
mkuse!{use rustc_expand :: base :: { Annotatable , DeriveResolution , ExpandResult , ExtCtxt , Indeterminate , MultiItemModifier , } ;}
mkuse!{use rustc_feature :: AttributeTemplate ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_span :: { ErrorGuaranteed , Ident , Span , sym } ;}
mkuse!{use crate :: cfg_eval :: cfg_eval ;}
mkuse!{use crate :: errors ;}
mkitem!{mkstruct!{pub (crate) struct Expander { pub is_const : bool , }}}
mkitem!{mkimpl!{impl MultiItemModifier for Expander { fn expand (& self , ecx : & mut ExtCtxt < '_ > , span : Span , meta_item : & ast :: MetaItem , item : Annotatable , _ : bool ,) -> ExpandResult < Vec < Annotatable > , Annotatable > { let sess = ecx . sess ; if report_bad_target (sess , & item , span) . is_err () { return ExpandResult :: Ready (vec ! [item]) ; } let (sess , features) = (ecx . sess , ecx . ecfg . features) ; let result = ecx . resolver . resolve_derives (ecx . current_expansion . id , ecx . force_mode , & | | { let template = AttributeTemplate { list : Some (& ["Trait1, Trait2, ..."]) , .. Default :: default () } ; validate_attr :: check_builtin_meta_item (& sess . psess , meta_item , ast :: AttrStyle :: Outer , sym :: derive , template , true ,) ; let mut resolutions = match & meta_item . kind { MetaItemKind :: List (list) => { list . iter () . filter_map (| meta_item_inner | match meta_item_inner { MetaItemInner :: MetaItem (meta) => Some (meta) , MetaItemInner :: Lit (lit) => { report_unexpected_meta_item_lit (sess , lit) ; None } }) . map (| meta | { report_path_args (sess , meta) ; meta . path . clone () }) . map (| path | DeriveResolution { path , item : dummy_annotatable () , exts : None , is_const : self . is_const , }) . collect () } _ => vec ! [] , } ; match & mut resolutions [..] { [] => { } [first , others @ ..] => { first . item = cfg_eval (sess , features , item . clone () , ecx . current_expansion . lint_node_id ,) ; for other in others { other . item = first . item . clone () ; } } } resolutions }) ; match result { Ok (()) => ExpandResult :: Ready (vec ! [item]) , Err (Indeterminate) => ExpandResult :: Retry (item) , } } }}}

macro_rules! dummy_annotatable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dummy_annotatable in module {}", module_path!());
    };
}

mkfn!{
    dummy_annotatable_introspect!();
    fn dummy_annotatable () -> Annotatable { Annotatable :: GenericParam (ast :: GenericParam { id : ast :: DUMMY_NODE_ID , ident : Ident :: dummy () , attrs : Default :: default () , bounds : Default :: default () , is_placeholder : false , kind : GenericParamKind :: Lifetime , colon_span : None , }) }
}

macro_rules! report_bad_target_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_bad_target in module {}", module_path!());
    };
}

mkfn!{
    report_bad_target_introspect!();
    fn report_bad_target (sess : & Session , item : & Annotatable , span : Span ,) -> Result < () , ErrorGuaranteed > { let item_kind = match item { Annotatable :: Item (item) => Some (& item . kind) , Annotatable :: Stmt (stmt) => match & stmt . kind { StmtKind :: Item (item) => Some (& item . kind) , _ => None , } , _ => None , } ; let bad_target = ! matches ! (item_kind , Some (ItemKind :: Struct (..) | ItemKind :: Enum (..) | ItemKind :: Union (..))) ; if bad_target { return Err (sess . dcx () . emit_err (errors :: BadDeriveTarget { span , item : item . span () })) ; } Ok (()) }
}

macro_rules! report_unexpected_meta_item_lit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_unexpected_meta_item_lit in module {}", module_path!());
    };
}

mkfn!{
    report_unexpected_meta_item_lit_introspect!();
    fn report_unexpected_meta_item_lit (sess : & Session , lit : & ast :: MetaItemLit) { let help = match lit . kind { ast :: LitKind :: Str (_ , ast :: StrStyle :: Cooked) if rustc_lexer :: is_ident (lit . symbol . as_str ()) => { errors :: BadDeriveLitHelp :: StrLit { sym : lit . symbol } } _ => errors :: BadDeriveLitHelp :: Other , } ; sess . dcx () . emit_err (errors :: BadDeriveLit { span : lit . span , help }) ; }
}

macro_rules! report_path_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_path_args in module {}", module_path!());
    };
}

mkfn!{
    report_path_args_introspect!();
    fn report_path_args (sess : & Session , meta : & ast :: MetaItem) { let span = meta . span . with_lo (meta . path . span . hi ()) ; match meta . kind { MetaItemKind :: Word => { } MetaItemKind :: List (..) => { sess . dcx () . emit_err (errors :: DerivePathArgsList { span }) ; } MetaItemKind :: NameValue (..) => { sess . dcx () . emit_err (errors :: DerivePathArgsValue { span }) ; } } }
}