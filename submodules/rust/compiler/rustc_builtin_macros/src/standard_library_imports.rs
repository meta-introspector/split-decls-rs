mkuse!{use rustc_ast :: { self as ast , attr } ;}
mkuse!{use rustc_expand :: base :: { ExtCtxt , ResolverExpand } ;}
mkuse!{use rustc_expand :: expand :: ExpansionConfig ;}
mkuse!{use rustc_feature :: Features ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_span :: edition :: Edition :: * ;}
mkuse!{use rustc_span :: hygiene :: AstPass ;}
mkuse!{use rustc_span :: { DUMMY_SP , Ident , Symbol , kw , sym } ;}
mkuse!{use thin_vec :: thin_vec ;}

macro_rules! inject_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function inject in module {}", module_path!());
    };
}

mkfn!{
    inject_introspect!();
    pub fn inject (krate : & mut ast :: Crate , pre_configured_attrs : & [ast :: Attribute] , resolver : & mut dyn ResolverExpand , sess : & Session , features : & Features ,) -> usize { let orig_num_items = krate . items . len () ; let edition = sess . psess . edition ; let name : Symbol = if attr :: contains_name (pre_configured_attrs , sym :: no_core) { return 0 ; } else if attr :: contains_name (pre_configured_attrs , sym :: no_std) { sym :: core } else { sym :: std } ; let expn_id = resolver . expansion_for_ast_pass (DUMMY_SP , AstPass :: StdImports , & [sym :: prelude_import] , None ,) ; let span = DUMMY_SP . with_def_site_ctxt (expn_id . to_expn_id ()) ; let call_site = DUMMY_SP . with_call_site_ctxt (expn_id . to_expn_id ()) ; let ecfg = ExpansionConfig :: default (sym :: std_lib_injection , features) ; let cx = ExtCtxt :: new (sess , ecfg , resolver , None) ; let ident_span = if edition >= Edition2018 { span } else { call_site } ; let item = cx . item (span , thin_vec ! [cx . attr_word (sym :: macro_use , span)] , ast :: ItemKind :: ExternCrate (None , Ident :: new (name , ident_span)) ,) ; let root = (edition == Edition2015) . then_some (kw :: PathRoot) ; let import_path = root . iter () . chain (& [name , sym :: prelude]) . chain (& [match edition { Edition2015 => sym :: rust_2015 , Edition2018 => sym :: rust_2018 , Edition2021 => sym :: rust_2021 , Edition2024 => sym :: rust_2024 , EditionFuture => sym :: rust_future , }]) . map (| & symbol | Ident :: new (symbol , span)) . collect () ; let use_item = cx . item (span , thin_vec ! [cx . attr_word (sym :: prelude_import , span)] , ast :: ItemKind :: Use (ast :: UseTree { prefix : cx . path (span , import_path) , kind : ast :: UseTreeKind :: Glob , span , }) ,) ; krate . items . splice (0 .. 0 , [item , use_item]) ; krate . items . len () - orig_num_items }
}