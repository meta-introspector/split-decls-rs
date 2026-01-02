mkuse!{use rustc_errors :: MultiSpan ;}
mkuse!{use rustc_hir :: def :: { DefKind , Res } ;}
mkuse!{use rustc_hir :: intravisit :: { self , Visitor , VisitorExt } ;}
mkuse!{use rustc_hir :: { Body , HirId , Item , ItemKind , Node , Path , TyKind } ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_session :: { declare_lint , impl_lint_pass } ;}
mkuse!{use rustc_span :: def_id :: { DefId , LOCAL_CRATE } ;}
mkuse!{use rustc_span :: { ExpnKind , Span , kw , sym } ;}
mkuse!{use crate :: lints :: { NonLocalDefinitionsCargoUpdateNote , NonLocalDefinitionsDiag } ;}
mkuse!{use crate :: { LateContext , LateLintPass , LintContext , fluent_generated as fluent } ;}
mkitem!{declare_lint ! { # [doc = " The `non_local_definitions` lint checks for `impl` blocks and `#[macro_export]`"] # [doc = " macro inside bodies (functions, enum discriminant, ...)."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " #![warn(non_local_definitions)]"] # [doc = " trait MyTrait {}"] # [doc = " struct MyStruct;"] # [doc = ""] # [doc = " fn foo() {"] # [doc = "     impl MyTrait for MyStruct {}"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Creating non-local definitions go against expectation and can create discrepancies"] # [doc = " in tooling. It should be avoided. It may become deny-by-default in edition 2024"] # [doc = " and higher, see the tracking issue <https://github.com/rust-lang/rust/issues/120363>."] # [doc = ""] # [doc = " An `impl` definition is non-local if it is nested inside an item and neither"] # [doc = " the type nor the trait are at the same nesting level as the `impl` block."] # [doc = ""] # [doc = " All nested bodies (functions, enum discriminant, array length, consts) (expect for"] # [doc = " `const _: Ty = { ... }` in top-level module, which is still undecided) are checked."] pub NON_LOCAL_DEFINITIONS , Warn , "checks for non-local definitions" , report_in_external_macro }}
mkitem!{mkstruct!{# [derive (Default)] pub (crate) struct NonLocalDefinitions { body_depth : u32 , }}}
mkitem!{impl_lint_pass ! (NonLocalDefinitions => [NON_LOCAL_DEFINITIONS]) ;}
mkitem!{mkimpl!{impl < 'tcx > LateLintPass < 'tcx > for NonLocalDefinitions { fn check_body (& mut self , _cx : & LateContext < 'tcx > , _body : & Body < 'tcx >) { self . body_depth += 1 ; } fn check_body_post (& mut self , _cx : & LateContext < 'tcx > , _body : & Body < 'tcx >) { self . body_depth -= 1 ; } fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < 'tcx >) { if self . body_depth == 0 { return ; } let def_id = item . owner_id . def_id . into () ; let parent = cx . tcx . parent (def_id) ; let parent_def_kind = cx . tcx . def_kind (parent) ; let parent_opt_item_name = cx . tcx . opt_item_name (parent) ; if self . body_depth == 1 && parent_def_kind == DefKind :: Const && parent_opt_item_name == Some (kw :: Underscore) { return ; } let cargo_update = | | { let oexpn = item . span . ctxt () . outer_expn_data () ; if let Some (def_id) = oexpn . macro_def_id && let ExpnKind :: Macro (macro_kind , macro_name) = oexpn . kind && def_id . krate != LOCAL_CRATE && rustc_session :: utils :: was_invoked_from_cargo () { Some (NonLocalDefinitionsCargoUpdateNote { macro_kind : macro_kind . descr () , macro_name , crate_name : cx . tcx . crate_name (def_id . krate) , }) } else { None } } ; let is_at_toplevel_doctest = | | { self . body_depth == 2 && cx . tcx . env_var_os ("UNSTABLE_RUSTDOC_TEST_PATH" . as_ref ()) . is_some () } ; match item . kind { ItemKind :: Impl (impl_) => { let mut collector = PathCollector { paths : Vec :: new () } ; collector . visit_ty_unambig (& impl_ . self_ty) ; if let Some (of_trait) = impl_ . of_trait { collector . visit_trait_ref (& of_trait . trait_ref) ; } collector . paths . retain (| p | matches ! (p . res , Res :: Def (def_kind , _) if def_kind != DefKind :: TyParam) ,) ; let outermost_impl_parent = peel_parent_while (cx . tcx , parent , | tcx , did | { tcx . def_kind (did) == DefKind :: Mod || (tcx . def_kind (did) == DefKind :: Const && tcx . opt_item_name (did) == Some (kw :: Underscore)) }) ; if collector . paths . iter () . any (| path | path_has_local_parent (path , cx , parent , outermost_impl_parent)) { return ; } let span_for_const_anon_suggestion = if parent_def_kind == DefKind :: Const && parent_opt_item_name != Some (kw :: Underscore) && let Some (parent) = parent . as_local () && let Node :: Item (item) = cx . tcx . hir_node_by_def_id (parent) && let ItemKind :: Const (ident , _ , ty , _) = item . kind && let TyKind :: Tup (& []) = ty . kind { Some (ident . span) } else { None } ; let const_anon = matches ! (parent_def_kind , DefKind :: Const | DefKind :: Static { .. }) . then_some (span_for_const_anon_suggestion) ; let impl_span = item . span . shrink_to_lo () . to (impl_ . self_ty . span) ; let mut ms = MultiSpan :: from_span (impl_span) ; for path in & collector . paths { # [allow (rustc :: untranslatable_diagnostic)] ms . push_span_label (path_span_without_args (path) , format ! ("`{}` is not local" , path_name_to_string (path)) ,) ; } let doctest = is_at_toplevel_doctest () ; if ! doctest { ms . push_span_label (cx . tcx . def_span (parent) , fluent :: lint_non_local_definitions_impl_move_help ,) ; } let macro_to_change = if let ExpnKind :: Macro (kind , name) = item . span . ctxt () . outer_expn_data () . kind { Some ((name . to_string () , kind . descr ())) } else { None } ; cx . emit_span_lint (NON_LOCAL_DEFINITIONS , ms , NonLocalDefinitionsDiag :: Impl { depth : self . body_depth , body_kind_descr : cx . tcx . def_kind_descr (parent_def_kind , parent) , body_name : parent_opt_item_name . map (| s | s . to_ident_string ()) . unwrap_or_else (| | "<unnameable>" . to_string ()) , cargo_update : cargo_update () , const_anon , doctest , macro_to_change , } ,) } ItemKind :: Macro (_ , _macro , _kinds) if cx . tcx . has_attr (item . owner_id . def_id , sym :: macro_export) => { cx . emit_span_lint (NON_LOCAL_DEFINITIONS , item . span , NonLocalDefinitionsDiag :: MacroRules { depth : self . body_depth , body_kind_descr : cx . tcx . def_kind_descr (parent_def_kind , parent) , body_name : parent_opt_item_name . map (| s | s . to_ident_string ()) . unwrap_or_else (| | "<unnameable>" . to_string ()) , cargo_update : cargo_update () , doctest : is_at_toplevel_doctest () , } ,) } _ => { } } } }}}
mkitem!{mkstruct!{# [doc = " Simple hir::Path collector"] struct PathCollector < 'tcx > { paths : Vec < Path < 'tcx > > , }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for PathCollector < 'tcx > { fn visit_path (& mut self , path : & Path < 'tcx > , _id : HirId) { self . paths . push (path . clone ()) ; intravisit :: walk_path (self , path) } }}}

macro_rules! path_has_local_parent_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function path_has_local_parent in module {}", module_path!());
    };
}

mkfn!{
    path_has_local_parent_introspect!();
    # [doc = " Given a path, this checks if the if the parent resolution def id corresponds to"] # [doc = " the def id of the parent impl definition (the direct one and the outermost one)."] # [doc = ""] # [doc = " Given this path, we will look at the path (and ignore any generic args):"] # [doc = ""] # [doc = " ```text"] # [doc = "    std::convert::PartialEq<Foo<Bar>>"] # [doc = "    ^^^^^^^^^^^^^^^^^^^^^^^"] # [doc = " ```"] # [inline] fn path_has_local_parent (path : & Path < '_ > , cx : & LateContext < '_ > , impl_parent : DefId , outermost_impl_parent : Option < DefId > ,) -> bool { path . res . opt_def_id () . is_some_and (| did | did_has_local_parent (did , cx . tcx , impl_parent , outermost_impl_parent)) }
}

macro_rules! did_has_local_parent_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function did_has_local_parent in module {}", module_path!());
    };
}

mkfn!{
    did_has_local_parent_introspect!();
    # [doc = " Given a def id this checks if the parent def id (modulo modules) correspond to"] # [doc = " the def id of the parent impl definition (the direct one and the outermost one)."] # [inline] fn did_has_local_parent (did : DefId , tcx : TyCtxt < '_ > , impl_parent : DefId , outermost_impl_parent : Option < DefId > ,) -> bool { if ! did . is_local () { return false ; } let Some (parent_did) = tcx . opt_parent (did) else { return false ; } ; peel_parent_while (tcx , parent_did , | tcx , did | { tcx . def_kind (did) == DefKind :: Mod || (tcx . def_kind (did) == DefKind :: Const && tcx . opt_item_name (did) == Some (kw :: Underscore)) }) . map (| parent_did | parent_did == impl_parent || Some (parent_did) == outermost_impl_parent) . unwrap_or (false) }
}

macro_rules! peel_parent_while_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function peel_parent_while in module {}", module_path!());
    };
}

mkfn!{
    peel_parent_while_introspect!();
    # [doc = " Given a `DefId` checks if it satisfies `f` if it does check with it's parent and continue"] # [doc = " until it doesn't satisfies `f` and return the last `DefId` checked."] # [doc = ""] # [doc = " In other word this method return the first `DefId` that doesn't satisfies `f`."] # [inline] fn peel_parent_while (tcx : TyCtxt < '_ > , mut did : DefId , mut f : impl FnMut (TyCtxt < '_ > , DefId) -> bool ,) -> Option < DefId > { while ! did . is_crate_root () && f (tcx , did) { did = tcx . opt_parent (did) . filter (| parent_did | parent_did . is_local ()) ? ; } Some (did) }
}

macro_rules! path_span_without_args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function path_span_without_args in module {}", module_path!());
    };
}

mkfn!{
    path_span_without_args_introspect!();
    # [doc = " Return for a given `Path` the span until the last args"] fn path_span_without_args (path : & Path < '_ >) -> Span { if let Some (args) = & path . segments . last () . unwrap () . args { path . span . until (args . span_ext) } else { path . span } }
}

macro_rules! path_name_to_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function path_name_to_string in module {}", module_path!());
    };
}

mkfn!{
    path_name_to_string_introspect!();
    # [doc = " Return a \"error message-able\" ident for the last segment of the `Path`"] fn path_name_to_string (path : & Path < '_ >) -> String { path . segments . last () . unwrap () . ident . to_string () }
}