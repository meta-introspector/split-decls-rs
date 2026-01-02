mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkmod!{state, { 
                getname!(state);
                getsrc!(state);
                getpath!(state);
                get_deps!(state);
                get_crates!(state);
                mkinclude!(state);
                 
            }}
mkuse!{use std :: borrow :: Cow ;}
mkuse!{use rustc_ast as ast ;}
mkuse!{use rustc_ast :: token :: { Token , TokenKind } ;}
mkuse!{use rustc_ast :: tokenstream :: { TokenStream , TokenTree } ;}
mkuse!{pub use state :: { AnnNode , Comments , PpAnn , PrintState , State , print_crate , print_crate_as_interface , } ;}

macro_rules! token_kind_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function token_kind_to_string in module {}", module_path!());
    };
}

mkfn!{
    token_kind_to_string_introspect!();
    # [doc = " Print the token kind precisely, without converting `$crate` into its respective crate name."] pub fn token_kind_to_string (tok : & TokenKind) -> Cow < 'static , str > { State :: new () . token_kind_to_string (tok) }
}

macro_rules! token_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function token_to_string in module {}", module_path!());
    };
}

mkfn!{
    token_to_string_introspect!();
    # [doc = " Print the token precisely, without converting `$crate` into its respective crate name."] pub fn token_to_string (token : & Token) -> Cow < 'static , str > { State :: new () . token_to_string (token) }
}

macro_rules! ty_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ty_to_string in module {}", module_path!());
    };
}

mkfn!{
    ty_to_string_introspect!();
    pub fn ty_to_string (ty : & ast :: Ty) -> String { State :: new () . ty_to_string (ty) }
}

macro_rules! bounds_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function bounds_to_string in module {}", module_path!());
    };
}

mkfn!{
    bounds_to_string_introspect!();
    pub fn bounds_to_string (bounds : & [ast :: GenericBound]) -> String { State :: new () . bounds_to_string (bounds) }
}

macro_rules! where_bound_predicate_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function where_bound_predicate_to_string in module {}", module_path!());
    };
}

mkfn!{
    where_bound_predicate_to_string_introspect!();
    pub fn where_bound_predicate_to_string (where_bound_predicate : & ast :: WhereBoundPredicate) -> String { State :: new () . where_bound_predicate_to_string (where_bound_predicate) }
}

macro_rules! pat_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function pat_to_string in module {}", module_path!());
    };
}

mkfn!{
    pat_to_string_introspect!();
    pub fn pat_to_string (pat : & ast :: Pat) -> String { State :: new () . pat_to_string (pat) }
}

macro_rules! expr_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function expr_to_string in module {}", module_path!());
    };
}

mkfn!{
    expr_to_string_introspect!();
    pub fn expr_to_string (e : & ast :: Expr) -> String { State :: new () . expr_to_string (e) }
}

macro_rules! tt_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function tt_to_string in module {}", module_path!());
    };
}

mkfn!{
    tt_to_string_introspect!();
    pub fn tt_to_string (tt : & TokenTree) -> String { State :: new () . tt_to_string (tt) }
}

macro_rules! tts_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function tts_to_string in module {}", module_path!());
    };
}

mkfn!{
    tts_to_string_introspect!();
    pub fn tts_to_string (tokens : & TokenStream) -> String { State :: new () . tts_to_string (tokens) }
}

macro_rules! item_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function item_to_string in module {}", module_path!());
    };
}

mkfn!{
    item_to_string_introspect!();
    pub fn item_to_string (i : & ast :: Item) -> String { State :: new () . item_to_string (i) }
}

macro_rules! assoc_item_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function assoc_item_to_string in module {}", module_path!());
    };
}

mkfn!{
    assoc_item_to_string_introspect!();
    pub fn assoc_item_to_string (i : & ast :: AssocItem) -> String { State :: new () . assoc_item_to_string (i) }
}

macro_rules! foreign_item_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function foreign_item_to_string in module {}", module_path!());
    };
}

mkfn!{
    foreign_item_to_string_introspect!();
    pub fn foreign_item_to_string (i : & ast :: ForeignItem) -> String { State :: new () . foreign_item_to_string (i) }
}

macro_rules! stmt_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function stmt_to_string in module {}", module_path!());
    };
}

mkfn!{
    stmt_to_string_introspect!();
    pub fn stmt_to_string (s : & ast :: Stmt) -> String { State :: new () . stmt_to_string (s) }
}

macro_rules! path_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function path_to_string in module {}", module_path!());
    };
}

mkfn!{
    path_to_string_introspect!();
    pub fn path_to_string (p : & ast :: Path) -> String { State :: new () . path_to_string (p) }
}

macro_rules! path_segment_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function path_segment_to_string in module {}", module_path!());
    };
}

mkfn!{
    path_segment_to_string_introspect!();
    pub fn path_segment_to_string (p : & ast :: PathSegment) -> String { State :: new () . path_segment_to_string (p) }
}

macro_rules! vis_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vis_to_string in module {}", module_path!());
    };
}

mkfn!{
    vis_to_string_introspect!();
    pub fn vis_to_string (v : & ast :: Visibility) -> String { State :: new () . vis_to_string (v) }
}

macro_rules! meta_list_item_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function meta_list_item_to_string in module {}", module_path!());
    };
}

mkfn!{
    meta_list_item_to_string_introspect!();
    pub fn meta_list_item_to_string (li : & ast :: MetaItemInner) -> String { State :: new () . meta_list_item_to_string (li) }
}

macro_rules! attribute_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function attribute_to_string in module {}", module_path!());
    };
}

mkfn!{
    attribute_to_string_introspect!();
    pub fn attribute_to_string (attr : & ast :: Attribute) -> String { State :: new () . attribute_to_string (attr) }
}

macro_rules! to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function to_string in module {}", module_path!());
    };
}

mkfn!{
    to_string_introspect!();
    pub fn to_string (f : impl FnOnce (& mut State < '_ >)) -> String { State :: to_string (f) }
}

macro_rules! crate_to_string_for_macros_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function crate_to_string_for_macros in module {}", module_path!());
    };
}

mkfn!{
    crate_to_string_for_macros_introspect!();
    pub fn crate_to_string_for_macros (krate : & ast :: Crate) -> String { State :: to_string (| s | { s . print_inner_attributes (& krate . attrs) ; for item in & krate . items { s . print_item (item) ; } }) }
}