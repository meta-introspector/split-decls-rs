mkuse!{use rustc_ast :: LitKind ;}
mkuse!{use rustc_ast :: attr :: AttributeExt ;}
mkuse!{use rustc_feature :: is_builtin_attr_name ;}
mkuse!{use rustc_hir :: RustcVersion ;}
mkuse!{use rustc_span :: { Symbol , sym } ;}
mkuse!{use crate :: context :: { AcceptContext , Stage } ;}
mkuse!{use crate :: parser :: ArgParser ;}

macro_rules! parse_version_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_version in module {}", module_path!());
    };
}

mkfn!{
    parse_version_introspect!();
    # [doc = " Parse a rustc version number written inside string literal in an attribute,"] # [doc = " like appears in `since = \"1.0.0\"`. Suffixes like \"-dev\" and \"-nightly\" are"] # [doc = " not accepted in this position, unlike when parsing CFG_RELEASE."] pub fn parse_version (s : Symbol) -> Option < RustcVersion > { let mut components = s . as_str () . split ('-') ; let d = components . next () ? ; if components . next () . is_some () { return None ; } let mut digits = d . splitn (3 , '.') ; let major = digits . next () ? . parse () . ok () ? ; let minor = digits . next () ? . parse () . ok () ? ; let patch = digits . next () . unwrap_or ("0") . parse () . ok () ? ; Some (RustcVersion { major , minor , patch }) }
}

macro_rules! is_builtin_attr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_builtin_attr in module {}", module_path!());
    };
}

mkfn!{
    is_builtin_attr_introspect!();
    pub fn is_builtin_attr (attr : & impl AttributeExt) -> bool { attr . is_doc_comment () || attr . ident () . is_some_and (| ident | is_builtin_attr_name (ident . name)) }
}

macro_rules! is_doc_alias_attrs_contain_symbol_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_doc_alias_attrs_contain_symbol in module {}", module_path!());
    };
}

mkfn!{
    is_doc_alias_attrs_contain_symbol_introspect!();
    pub fn is_doc_alias_attrs_contain_symbol < 'tcx , T : AttributeExt + 'tcx > (attrs : impl Iterator < Item = & 'tcx T > , symbol : Symbol ,) -> bool { let doc_attrs = attrs . filter (| attr | attr . has_name (sym :: doc)) ; for attr in doc_attrs { let Some (values) = attr . meta_item_list () else { continue ; } ; let alias_values = values . iter () . filter (| v | v . has_name (sym :: alias)) ; for v in alias_values { if let Some (nested) = v . meta_item_list () { let mut iter = nested . iter () . filter_map (| item | item . lit ()) . map (| item | item . symbol) ; if iter . any (| s | s == symbol) { return true ; } } else if let Some (meta) = v . meta_item () && let Some (lit) = meta . name_value_literal () { if lit . symbol == symbol { return true ; } } } } false }
}

macro_rules! parse_single_integer_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function parse_single_integer in module {}", module_path!());
    };
}

mkfn!{
    parse_single_integer_introspect!();
    # [doc = " Parse a single integer."] # [doc = ""] # [doc = " Used by attributes that take a single integer as argument, such as"] # [doc = " `#[link_ordinal]` and `#[rustc_layout_scalar_valid_range_start]`."] # [doc = " `cx` is the context given to the attribute."] # [doc = " `args` is the parser for the attribute arguments."] pub (crate) fn parse_single_integer < S : Stage > (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ > ,) -> Option < u128 > { let Some (list) = args . list () else { cx . expected_list (cx . attr_span) ; return None ; } ; let Some (single) = list . single () else { cx . expected_single_argument (list . span) ; return None ; } ; let Some (lit) = single . lit () else { cx . expected_integer_literal (single . span ()) ; return None ; } ; let LitKind :: Int (num , _ty) = lit . kind else { cx . expected_integer_literal (single . span ()) ; return None ; } ; Some (num . 0) }
}